package com.ghostlink.app.crypto

import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class KeyBundleManager @Inject constructor(
    private val signalManager: SignalManager
) {
    // Manages active signed pre-keys renewal cycle
}
