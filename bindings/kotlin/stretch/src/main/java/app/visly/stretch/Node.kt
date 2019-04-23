package app.visly.stretch
import androidx.annotation.Keep
import java.lang.ref.WeakReference

interface MeasureFunc {
    fun measure(constraints: Size<Float?>): Size<Float>
}

private class MeasureFuncImpl(val measureFunc: WeakReference<MeasureFunc>) {
    @Keep fun measure(width: Float, height: Float): FloatArray {
        val result = measureFunc.get()!!.measure(Size(
            if (width.isNaN()) null else width,
            if (height.isNaN()) null else height
        ))
        return floatArrayOf(result.width, result.height)
    }
}

class Node {
    companion object {
        init {
            Stretch.init()
        }
    }

    private val rustptr: Long
    private var style: Style
    private var children: MutableList<Node>
    private var measure: MeasureFunc? = null

    constructor(style: Style, measure: MeasureFunc) {
        this.rustptr = nConstructLeaf(style.rustptr, MeasureFuncImpl(WeakReference(measure)))
        this.style = style
        this.children = mutableListOf()
        this.measure = measure
    }

    constructor(style: Style, children: List<Node>) {
        this.rustptr = nConstruct(style.rustptr, LongArray(children.size) { children[it].rustptr })
        this.style = style
        this.children = children.toMutableList()
    }

    protected fun finalize() {
        nFree(rustptr)
    }

    fun setMeasure(measure: MeasureFunc) {
        nSetMeasure(rustptr, MeasureFuncImpl(WeakReference(measure)))
        this.measure = measure
    }

    fun getChildren(): List<Node> {
        return this.children
    }

    fun setChildren(children: List<Node>) {
        nSetChildren(rustptr, LongArray(children.size) { children[it].rustptr })
        this.children = children.toMutableList()
    }

    fun addChild(child: Node) {
        nAddChild(rustptr, child.rustptr)
        children.add(child)
    }

    fun replaceChildAtIndex(index: Int, child: Node): Node {
        nReplaceChildAtIndex(rustptr, index, child.rustptr)
        return children.set(index, child)
    }

    fun removeChild(child: Node): Node {
        nRemoveChild(rustptr, child.rustptr)
        children.remove(child)
        return child
    }

    fun removeChildAtIndex(index: Int): Node {
        nRemoveChildAtIndex(rustptr, index)
        return children.removeAt(index)
    }

    fun getStyle(): Style {
        return this.style
    }

    fun setStyle(style: Style) {
        nSetStyle(rustptr, style.rustptr)
        this.style = style
    }

    fun markDirty() {
        nMarkDirty(rustptr)
    }

    fun isDirty(): Boolean {
        return nIsDirty(rustptr)
    }

    fun getChildCount(): Int {
        return children.size
    }

    fun computeLayout(size: Size<Float?>): Layout {
        val result = Layout.fromFloatArray(nComputeLayout(rustptr, size.width ?: Float.NaN, size.height ?: Float.NaN), 0)
        return result.second
    }

    private external fun nConstruct(style: Long, children: LongArray): Long
    private external fun nConstructLeaf(style: Long, measure: MeasureFuncImpl): Long
    private external fun nFree(ptr: Long)
    private external fun nSetMeasure(ptr: Long, measure: MeasureFuncImpl)
    private external fun nSetChildren(ptr: Long, children: LongArray)
    private external fun nAddChild(ptr: Long, child: Long)
    private external fun nReplaceChildAtIndex(ptr: Long, index: Int, child: Long): Long
    private external fun nRemoveChild(ptr: Long, child: Long): Long
    private external fun nRemoveChildAtIndex(ptr: Long, index: Int): Long
    private external fun nSetStyle(ptr: Long, args: Long): Boolean
    private external fun nIsDirty(ptr: Long): Boolean
    private external fun nMarkDirty(ptr: Long)
    private external fun nComputeLayout(ptr: Long, width: Float, height: Float): FloatArray
}