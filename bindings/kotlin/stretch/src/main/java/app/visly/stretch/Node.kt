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
        this.rustptr = nConstructLeaf(Stretch.ptr, style.rustptr, MeasureFuncImpl(WeakReference(measure)))
        this.style = style
        this.children = mutableListOf()
        this.measure = measure
    }

    constructor(style: Style, children: List<Node>) {
        this.rustptr = nConstruct(Stretch.ptr, style.rustptr, LongArray(children.size) { children[it].rustptr })
        this.style = style
        this.children = children.toMutableList()
    }

    fun free() {
        style.free()
        nFree(Stretch.ptr, rustptr)
    }

    fun freeNodes() {
        children.forEach {
            if (it.getChildCount() > 0) {
                it.freeNodes()
            }
        }
        free()
    }

    fun setMeasure(measure: MeasureFunc) {
        nSetMeasure(Stretch.ptr, rustptr, MeasureFuncImpl(WeakReference(measure)))
        this.measure = measure
    }

    fun getChildren(): List<Node> {
        return this.children
    }

    fun setChildren(children: List<Node>) {
        nSetChildren(Stretch.ptr, rustptr, LongArray(children.size) { children[it].rustptr })
        this.children = children.toMutableList()
    }

    fun addChild(child: Node) {
        nAddChild(Stretch.ptr, rustptr, child.rustptr)
        children.add(child)
    }

    fun replaceChildAtIndex(index: Int, child: Node): Node {
        nReplaceChildAtIndex(Stretch.ptr, rustptr, index, child.rustptr)
        return children.set(index, child)
    }

    fun removeChild(child: Node): Node {
        nRemoveChild(Stretch.ptr, rustptr, child.rustptr)
        children.remove(child)
        return child
    }

    fun removeChildAtIndex(index: Int): Node {
        nRemoveChildAtIndex(Stretch.ptr, rustptr, index)
        return children.removeAt(index)
    }

    fun getStyle(): Style {
        return this.style
    }

    fun setStyle(style: Style) {
        nSetStyle(Stretch.ptr, rustptr, style.rustptr)
        this.style = style
    }

    fun markDirty() {
        nMarkDirty(Stretch.ptr, rustptr)
    }

    fun isDirty(): Boolean {
        return nIsDirty(Stretch.ptr, rustptr)
    }

    fun getChildCount(): Int {
        return children.size
    }

    fun computeLayout(size: Size<Float?>): Layout {
        val result = Layout.fromFloatArray(nComputeLayout(Stretch.ptr, rustptr, size.width ?: Float.NaN, size.height ?: Float.NaN), 0)
        return result.second
    }

    private external fun nConstruct(stretch: Long, style: Long, children: LongArray): Long
    private external fun nConstructLeaf(stretch: Long, style: Long, measure: MeasureFuncImpl): Long
    private external fun nFree(stretch: Long, ptr: Long)
    private external fun nSetMeasure(stretch: Long, ptr: Long, measure: MeasureFuncImpl)
    private external fun nSetChildren(stretch: Long, ptr: Long, children: LongArray)
    private external fun nAddChild(stretch: Long, ptr: Long, child: Long)
    private external fun nReplaceChildAtIndex(stretch: Long, ptr: Long, index: Int, child: Long): Long
    private external fun nRemoveChild(stretch: Long, ptr: Long, child: Long): Long
    private external fun nRemoveChildAtIndex(stretch: Long, ptr: Long, index: Int): Long
    private external fun nSetStyle(stretch: Long, ptr: Long, args: Long): Boolean
    private external fun nIsDirty(stretch: Long, ptr: Long): Boolean
    private external fun nMarkDirty(stretch: Long, ptr: Long)
    private external fun nComputeLayout(stretch: Long, ptr: Long, width: Float, height: Float): FloatArray
}