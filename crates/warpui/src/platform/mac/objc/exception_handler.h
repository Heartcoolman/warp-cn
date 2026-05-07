#import <Foundation/Foundation.h>

// Installs an uncaught Objective-C exception handler that captures the
// NSException name/reason/userInfo/callStackSymbols to
// `~/Library/Logs/<CFBundleName>/uncaught_exception.log` before the runtime
// aborts the process.
//
// Strictly additive: does not flip `NSApplicationCrashOnExceptions` or any
// other AppKit behavior switch — the existing rethrow path already triggers
// our handler on macOS 26.
//
// Idempotent and process-global (guarded by dispatch_once). The log path is
// resolved at install time so the handler itself does no allocation-heavy
// work while the runtime is mid-terminate.
void warp_install_uncaught_exception_handler(void);
