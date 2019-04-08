package app.visly.stretch

typealias MeasureFunc = (Size<Float?>) -> Size<Float>

class Node {
    companion object {
        init {
            System.loadLibrary("stretch")
        }
    }

    internal val ptr: Long
    private var style: Style
    private var children: MutableList<Node>

    constructor(style: Style, measure: MeasureFunc) {
        this.ptr = nConstructLeaf(style.toFloatArray(), measure)
        this.style = style
        this.children = mutableListOf()
    }

    constructor(style: Style, children: List<Node>) {
        this.ptr = nConstruct(style.toFloatArray(), LongArray(children.size) { children[it].ptr })
        this.style = style
        this.children = children.toMutableList()
    }

    fun getChildren(): List<Node> {
        return this.children
    }

    fun setChildren(children: List<Node>) {
        nSetChildren(ptr, LongArray(children.size) { children[it].ptr })
        this.children = children.toMutableList()
    }

    fun addChild(child: Node) {
        nAddChild(ptr, child.ptr)
        children.add(child)
    }

    fun replaceChildAtIndex(index: Int, child: Node): Node {
        nReplaceChildAtIndex(ptr, index, child.ptr)
        return children.set(index, child)
    }

    fun removeChild(child: Node): Node {
        nRemoveChild(ptr, child.ptr)
        children.remove(child)
        return child
    }

    fun removeChildAtIndex(index: Int): Node {
        nRemoveChildAtIndex(ptr, index)
        return children.removeAt(index)
    }

    fun getStyle(): Style {
        return this.style
    }

    fun setStyle(style: Style) {
        nSetStyle(ptr, style.toFloatArray())
        this.style = style
    }

    fun markDirty() {
        nMarkDirty(ptr)
    }

    fun isDirty(): Boolean {
        return nIsDirty(ptr)
    }

    fun computeLayout(size: Size<Float?>): Layout {
        val result = Layout.fromFloatArray(nComputeLayput(ptr, size.toFloatArray2()), 0)
        return result.second
    }

    external fun nConstruct(style: FloatArray, children: LongArray): Long
    external fun nConstructLeaf(style: FloatArray, measure: MeasureFunc): Long
    external fun nSetChildren(ptr: Long, children: LongArray)
    external fun nAddChild(ptr: Long, child: Long)
    external fun nReplaceChildAtIndex(ptr: Long, index: Int, child: Long): Long
    external fun nRemoveChild(ptr: Long, child: Long): Long
    external fun nRemoveChildAtIndex(ptr: Long, index: Int): Long
    external fun nSetStyle(ptr: Long, args: FloatArray): Boolean
    external fun nIsDirty(ptr: Long): Boolean
    external fun nMarkDirty(ptr: Long)
    external fun nComputeLayput(ptr: Long, args: FloatArray): FloatArray
}