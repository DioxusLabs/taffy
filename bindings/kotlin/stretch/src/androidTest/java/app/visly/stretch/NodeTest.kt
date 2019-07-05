package app.visly.stretch

import org.hamcrest.CoreMatchers.*
import org.hamcrest.MatcherAssert.*
import org.junit.Test

class NodeTest {

    @Test
    fun testing() {
        val node1 = Node(Style(), listOf())
        val node2 = Node(Style(), listOf())
        val node3 = Node(Style(), listOf())
        val node4 = Node(Style(), listOf())

        assertThat(node1.isDirty(), `is`(true))
        assertThat(node2.isDirty(), `is`(true))
        assertThat(node3.isDirty(), `is`(true))
        assertThat(node4.isDirty(), `is`(true))
    }

    @Test
    fun createNode() {
        val child = Node(Style(), listOf())
        val node = Node(Style(), listOf(child))
        assertThat(node.getChildCount(), `is`(1))
    }

    @Test
    fun createLeaf() {
        val node = Node(Style(), object: MeasureFunc {
            override fun measure(constraints: Size<Float?>): Size<Float> {
                return Size(100f, 100f)
            }
        })

        val layout = node.computeLayout(Size(null, null))
        assertThat(layout.width, `is`(100f))
        assertThat(layout.height, `is`(100f))
    }

    @Test
    fun setMeasure() {
        val node = Node(Style(), listOf())
        node.setMeasure(object: MeasureFunc {
            override fun measure(constraints: Size<Float?>): Size<Float> {
                return Size(100f, 100f)
            }
        })

        val layout = node.computeLayout(Size(null, null))
        assertThat(layout.width, `is`(100f))
        assertThat(layout.height, `is`(100f))
    }

    @Test
    fun addChild() {
        val node = Node(Style(), listOf())
        val child = Node(Style(), listOf())
        node.addChild(child)
        assertThat(node.getChildCount(), `is`(1))
    }

    @Test
    fun removeChild() {
        val child = Node(Style(), listOf())
        val node = Node(Style(), listOf(child))
        node.removeChild(child)
        assertThat(node.getChildCount(), `is`(0))
    }

    @Test
    fun removeChildAtIndex() {
        val child = Node(Style(), listOf())
        val node = Node(Style(), listOf(child))
        node.removeChildAtIndex(0)
        assertThat(node.getChildCount(), `is`(0))
    }

    @Test
    fun replaceChildAtIndex() {
        val child1 = Node(Style(size = Size(Dimension.Points(100f), Dimension.Points(100f))), listOf())
        val child2 = Node(Style(size = Size(Dimension.Points(200f), Dimension.Points(200f))), listOf())
        val node = Node(Style(), listOf(child1))
        node.replaceChildAtIndex(0, child2)

        val layout = node.computeLayout(Size(null, null))
        assertThat(layout.width, `is`(200f))
        assertThat(layout.height, `is`(200f))
    }

    @Test
    fun setStyle() {
        val node = Node(Style(), listOf())
        node.setStyle(Style())
    }

    @Test
    fun setChildren() {
        val node = Node(Style(), listOf())
        val child = Node(Style(), listOf())
        node.setChildren(listOf(child))
        assertThat(node.getChildCount(), `is`(1))
    }

    @Test
    fun markDirty() {
        val node = Node(Style(), listOf())
        node.computeLayout(Size(null, null))
        assertThat(node.isDirty(), `is`(false))
        node.markDirty()
        assertThat(node.isDirty(), `is`(true))
    }
}