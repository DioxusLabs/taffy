package app.visly.stretch

class Stretch {
    companion object {
        private var didInit = false
        internal fun init() {
            if (!didInit) {
                System.loadLibrary("stretch")
                didInit = true
            }
        }
    }
}