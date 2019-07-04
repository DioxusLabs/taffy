package app.visly.stretch

class Stretch {
    companion object {
        private var didInit = false
        var ptr: Long = 0

        internal fun init() {
            if (!didInit) {
                System.loadLibrary("stretch")
                didInit = true
                ptr = nInit()
            }
        }

        @JvmStatic private external fun nInit(): Long
    }
}