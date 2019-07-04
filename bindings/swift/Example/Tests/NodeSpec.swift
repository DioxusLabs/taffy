import Quick
import Nimble
import StretchKit

class NodeSpec: QuickSpec {
    override func spec() {
        it("will create node") {
            let child = Node(style: Style(), children: [])
            let node = Node(style: Style(), children: [child])
            expect(node.children.count) == 1
        }
        
        it("will create leaf") {
            let node = Node(style: Style()) { constraint in
                return Size(width: 100.0, height: 100.0)
            }

            let layout = node.computeLayout(thatFits: Size(width: nil, height: nil))
            expect(layout.width) == 100.0
            expect(layout.height) == 100.0
        }

        it("will set measure") {
            let node = Node(style: Style(), children: [])
            node.measure = { constraint in
                return Size(width: 100.0, height: 100.0)
            }

            let layout = node.computeLayout(thatFits: Size(width: nil, height: nil))
            expect(layout.width) == 100.0
            expect(layout.height) == 100.0
        }

        it("will add child") {
            let node = Node(style: Style(), children: [])
            let child = Node(style: Style(), children: [])
            node.addChild(child)
            expect(node.children.count) == 1
        }

        it("will remove child") {
            let child = Node(style: Style(), children: [])
            let node = Node(style: Style(), children: [child])
            node.removeChild(child)
            expect(node.children.count) == 0
        }

        it("will remove child at index") {
            let child = Node(style: Style(), children: [])
            let node = Node(style: Style(), children: [child])
            node.removeChild(at: 0)
            expect(node.children.count) == 0
        }

        it("will replace child at index") {
            let child1 = Node(style: Style(size: Size(width: .points(100.0), height: .points(100.0))), children: [])
            let child2 = Node(style: Style(size: Size(width: .points(200.0), height: .points(200.0))), children: [])
            let node = Node(style: Style(), children: [child1])
            node.replaceChild(child2, at: 0)

            let layout = node.computeLayout(thatFits: Size(width: nil, height: nil))
            expect(layout.width) == 200.0
            expect(layout.height) == 200.0
        }

        it("will set style") {
            let node = Node(style: Style(size: Size(width: .points(100.0), height: .points(100.0))), children: [])
            node.style = Style(size: Size(width: .points(200.0), height: .points(200.0)))

            let layout = node.computeLayout(thatFits: Size(width: nil, height: nil))
            expect(layout.width) == 200.0
            expect(layout.height) == 200.0
        }

        it("will set children") {
            let node = Node(style: Style(), children: [])
            let child = Node(style: Style(), children: [])
            node.children = [child]
            expect(node.children.count) == 1
        }

        it("will mark node dirty") {
            let node = Node(style: Style(), children: [])
            let _ = node.computeLayout(thatFits: Size(width: nil, height: nil))
            expect(node.dirty) == false
            node.markDirty()
            expect(node.dirty) == true
        }
    }
}
