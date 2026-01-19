bolt-quadtree
=============

High-performance loose quadtree implementation in Rust with optional Python bindings.

Crates
------
- Rust crate: `bolt-quadtree` (library name `quadtree`)
- Python extension: `bolt_quadtree`

Rust usage
----------
```rust
use quadtree::quadtree::{EntityTypeUpdate, QuadTree};
use quadtree::shapes::{Rectangle, ShapeEnum};

fn main() -> quadtree::QuadtreeResult<()> {
    let bounds = Rectangle::new(0.0, 0.0, 100.0, 100.0);
    let mut tree = QuadTree::new(bounds)?;

    tree.insert(1, ShapeEnum::Rectangle(Rectangle::new(10.0, 10.0, 5.0, 5.0)), None)?;
    tree.insert(2, ShapeEnum::Circle(quadtree::shapes::Circle::new(20.0, 20.0, 3.0)), None)?;

    let mut hits = Vec::new();
    tree.collisions(ShapeEnum::Rectangle(Rectangle::new(10.0, 10.0, 2.0, 2.0)), &mut hits)?;

    tree.relocate(1, ShapeEnum::Rectangle(Rectangle::new(30.0, 30.0, 5.0, 5.0)), EntityTypeUpdate::Preserve)?;
    Ok(())
}
```

Python usage
------------
```python
import bolt_quadtree as qt

bounds = qt.Rectangle(0.0, 0.0, 100.0, 100.0)
tree = qt.QuadTree(bounds)
tree.insert(1, qt.Rectangle(10.0, 10.0, 5.0, 5.0))
hits = tree.collisions(qt.Rectangle(10.0, 10.0, 2.0, 2.0))
```

License
-------
MIT. See LICENSE.
