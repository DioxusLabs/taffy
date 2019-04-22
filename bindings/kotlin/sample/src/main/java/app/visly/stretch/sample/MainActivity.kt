package app.visly.stretch.sample

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import app.visly.stretch.*

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val child1 = Node(Style(size = Size(Dimension.Points(100f), Dimension.Points(100f))), listOf())
        val child2 = Node(Style(size = Size(Dimension.Points(100f), Dimension.Points(100f))), listOf())
        val child3 = Node(Style(size = Size(Dimension.Points(100f), Dimension.Points(100f))), listOf())

        val node = Node(
            Style(
                flexDirection = FlexDirection.Column,
                size = Size(
                    width = Dimension.Points(100f),
                    height = Dimension.Undefined
                )
            ),
            listOf(child1, child2, child3)
        )

        val layout = node.computeLayout(Size(null, null))

        Log.d("visly-debug-log", "" + layout)
    }
}
