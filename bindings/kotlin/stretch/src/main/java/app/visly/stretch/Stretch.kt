package app.visly.stretch

// TODO
// - Change measure to be a weak ref to node instance which in turn calls passed in measure function implementation from java.
//   Problem with this is jni library currently does not support weak refs. This can be worked around by creating weak ref on java side.
// - Integration tests to ensure bindings don't break

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