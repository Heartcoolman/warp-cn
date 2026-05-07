#import "exception_handler.h"

#import <Foundation/Foundation.h>
#import <fcntl.h>
#import <limits.h>
#import <pthread.h>
#import <stdatomic.h>
#import <stdbool.h>
#import <stdio.h>
#import <string.h>
#import <time.h>
#import <unistd.h>

static NSUncaughtExceptionHandler *previousHandler = NULL;
// Resolved at install time so the handler does no path resolution / directory
// creation while the runtime is mid-terminate.
static NSString *exceptionLogPath = nil;
// Same path as a plain C string so the POSIX-only receipt path can write
// without touching Foundation. Cached at install time alongside the NSString.
static char gExceptionLogPathCStr[PATH_MAX] = {0};
// Reentrancy guard: if our own logging code throws (it shouldn't, but defense
// in depth), we chain straight to the previous handler instead of recursing.
static atomic_bool isHandlingException = false;

static void warp_uncaught_handler(NSException *exception);

static NSString *resolveLogPath(void) {
    NSString *bundleName = [[NSBundle mainBundle] objectForInfoDictionaryKey:@"CFBundleName"];
    if (bundleName.length == 0) {
        bundleName = @"Warp";
    }
    NSArray<NSString *> *libDirs =
        NSSearchPathForDirectoriesInDomains(NSLibraryDirectory, NSUserDomainMask, YES);
    if (libDirs.count == 0) {
        return nil;
    }
    NSString *logsDir = [[libDirs.firstObject stringByAppendingPathComponent:@"Logs"]
                          stringByAppendingPathComponent:bundleName];
    [[NSFileManager defaultManager] createDirectoryAtPath:logsDir
                              withIntermediateDirectories:YES
                                               attributes:nil
                                                    error:NULL];
    NSString *path = [logsDir stringByAppendingPathComponent:@"uncaught_exception.log"];
    if (![[NSFileManager defaultManager] fileExistsAtPath:path]) {
        [[NSFileManager defaultManager] createFileAtPath:path contents:nil attributes:nil];
    }
    return path;
}

static void chainToPreviousHandler(NSException *exception) {
    if (previousHandler != NULL && previousHandler != &warp_uncaught_handler) {
        previousHandler(exception);
    }
}

// Copy `value`'s UTF-8 bytes into `dst` up to `dstSize-1` chars + NUL. Safe
// against nil / NULL UTF8String. Caller-owned buffer; no Foundation memory
// is retained past return.
static void copyNSStringUTF8(NSString *value, char *dst, size_t dstSize) {
    if (dstSize == 0) {
        return;
    }
    dst[0] = '\0';
    if (value == nil) {
        return;
    }
    const char *src = [value UTF8String];
    if (src != NULL) {
        strlcpy(dst, src, dstSize);
    }
}

// Minimum-viable POSIX receipt: a single stack-allocated record written via
// `open(2)` + `write(2)`. Runs *before* the rich Objective-C block so even a
// heap-corrupting crash leaves a name/reason line behind. Idempotent against
// the rich path — they produce two distinct entries in the same log.
static void writePosixReceipt(NSException *exception) {
    if (gExceptionLogPathCStr[0] == '\0') {
        return;
    }

    char timestamp[64] = {0};
    time_t now = time(NULL);
    struct tm localTime;
    if (localtime_r(&now, &localTime) != NULL) {
        strftime(timestamp, sizeof(timestamp), "%Y-%m-%dT%H:%M:%S%z", &localTime);
    }
    if (timestamp[0] == '\0') {
        snprintf(timestamp, sizeof(timestamp), "%lld", (long long)now);
    }

    char threadName[64] = {0};
    pthread_getname_np(pthread_self(), threadName, sizeof(threadName));

    char exceptionName[192] = {0};
    char exceptionReason[512] = {0};
    copyNSStringUTF8(exception.name, exceptionName, sizeof(exceptionName));
    copyNSStringUTF8(exception.reason, exceptionReason, sizeof(exceptionReason));

    char buf[1024];
    int written = snprintf(
        buf,
        sizeof(buf),
        "=== uncaught NSException POSIX receipt @ %s ===\n"
        "pid     : %d\n"
        "thread  : %s\n"
        "name    : %s\n"
        "reason  : %s\n"
        "\n",
        timestamp,
        getpid(),
        threadName,
        exceptionName,
        exceptionReason);
    if (written <= 0) {
        return;
    }
    size_t len = (size_t)written < sizeof(buf) ? (size_t)written : sizeof(buf) - 1;

    int fd = open(gExceptionLogPathCStr, O_CREAT | O_APPEND | O_WRONLY, 0644);
    if (fd < 0) {
        return;
    }
    ssize_t ignored = write(fd, buf, len);
    (void)ignored;
    close(fd);
}

static void warp_uncaught_handler(NSException *exception) {
    bool expected = false;
    if (!atomic_compare_exchange_strong(&isHandlingException, &expected, true)) {
        chainToPreviousHandler(exception);
        return;
    }

    // Stack-only first. If the heap is hosed and the rich block aborts in
    // malloc, we still leave behind a "minimum viable receipt" — name +
    // reason are usually enough to triage.
    writePosixReceipt(exception);

    @try {
        @autoreleasepool {
            NSString *path = exceptionLogPath;
            if (path != nil) {
                // Build timestamp via plain libc to avoid NSDateFormatter
                // allocation pressure during termination.
                char timestamp[64] = {0};
                time_t now = time(NULL);
                struct tm localTime;
                if (localtime_r(&now, &localTime) != NULL) {
                    strftime(timestamp, sizeof(timestamp),
                             "%Y-%m-%dT%H:%M:%S%z", &localTime);
                }
                if (timestamp[0] == '\0') {
                    snprintf(timestamp, sizeof(timestamp), "%lld", (long long)now);
                }

                char threadName[64] = {0};
                pthread_getname_np(pthread_self(), threadName, sizeof(threadName));

                NSMutableString *entry = [NSMutableString stringWithCapacity:4096];
                [entry appendFormat:@"=== uncaught NSException @ %s ===\n", timestamp];
                [entry appendFormat:@"pid     : %d\n", getpid()];
                [entry appendFormat:@"thread  : %s (%@)\n", threadName,
                    [NSThread currentThread].name ?: @""];
                [entry appendFormat:@"name    : %@\n", exception.name];
                [entry appendFormat:@"reason  : %@\n", exception.reason];
                [entry appendFormat:@"userInfo: %@\n", exception.userInfo];
                NSArray<NSString *> *symbols = exception.callStackSymbols;
                if (symbols.count > 0) {
                    [entry appendString:@"stack   :\n"];
                    for (NSString *frame in symbols) {
                        [entry appendFormat:@"  %@\n", frame];
                    }
                } else {
                    [entry appendString:@"stack   : (no symbols)\n"];
                }
                [entry appendString:@"\n"];

                NSData *data = [entry dataUsingEncoding:NSUTF8StringEncoding];
                NSFileHandle *fh = [NSFileHandle fileHandleForWritingAtPath:path];
                if (fh != nil) {
                    @try {
                        [fh seekToEndOfFile];
                        [fh writeData:data];
                        [fh synchronizeFile];
                    } @finally {
                        [fh closeFile];
                    }
                }
            }
        }
    } @catch (NSException *innerExc) {
        (void)innerExc;
        // Hard fallback: never recurse, never allocate.
        static const char message[] =
            "warp uncaught_exception_handler failed while logging\n";
        (void)write(STDERR_FILENO, message, sizeof(message) - 1);
    }

    chainToPreviousHandler(exception);
}

void warp_install_uncaught_exception_handler(void) {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        @autoreleasepool {
            NSString *path = resolveLogPath();
            if (path != nil) {
                exceptionLogPath = [path copy];
                const char *pathCStr = [path UTF8String];
                if (pathCStr != NULL) {
                    strlcpy(gExceptionLogPathCStr, pathCStr,
                            sizeof(gExceptionLogPathCStr));
                }
            }
        }
        previousHandler = NSGetUncaughtExceptionHandler();
        NSSetUncaughtExceptionHandler(&warp_uncaught_handler);
    });
}
