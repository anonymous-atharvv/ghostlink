import UIKit
import IOSSecuritySuite

class AppDelegate: NSObject, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        let isJailbroken = IOSSecuritySuite.amIJailbroken()
        let isDebuggerAttached = IOSSecuritySuite.amIDebugged()

        if isJailbroken || isDebuggerAttached {
            print("WARNING: System integrity check failed! Active security audit threat detected.")
        }

        return true
    }
}
