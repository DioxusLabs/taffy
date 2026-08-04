# C API for Taffy (ctaffy)

Taffy is a flexible, high-performance, cross-platform UI layout library written in Rust.

This directory contains C bindings for Taffy. The API is a pure C API (no C++ features are used), and is designed to be easy to build other language bindings on top of. In addition to the documentation below, you may to read through the header file (`include/taffy.h`).

## Examples

There are readable examples in the examples directory.

Assuming you have Rust and Cargo installed (and a C compiler), then this should work to run the basic example:

```bash
git clone https://github.com/DioxusLabs/taffy.git
cd taffy/ctaffy/examples
./compile-basic.sh
./basic
```

## Naming Conventions

- Everything in the Taffy C API is prefixed with `Taffy`, except enum variant names which are prefixed with `TAFFY_`
- Structs and Enums are named in UpperCamelCase (e.g. `TaffyTree`, `TaffyStyle`)
- Functions begin with the name of the struct they apply to, followed by an underscore, followed by the name of the function in UpperCamelCase (e.g. `TaffyTree_New`, `TaffyStyle_SetFlexBasis`)
- Enum variants are SCREAMING_SNAKE_CASE

## Error Handling

Error handling is managed by the use of return codes and "result" structs. All functions in the API return either an `enum TaffyReturnCode` or one of the `struct Taffy*Result` structs (or `void` in the case of infallible operations that don't return anything).

### Return codes

Error handling is managed by the use of an enum `TaffyReturnCode`:

```c
typedef enum TaffyReturnCode {
  TAFFY_RETURN_CODE_OK,
  TAFFY_RETURN_CODE_NULL_STYLE_POINTER,
  TAFFY_RETURN_CODE_NULL_TREE_POINTER,
  // ... (see header file for full definition)
} TaffyReturnCode;
```

`TAFFY_RETURN_CODE_OK` indicates that the operation succeeded. All other variant indicate

### Result structs

"Result structs" are used for functions that need to return another value in addition to a `TaffyReturnCode` indicating success/failure (such as style getters which need to return the relevant style value). As C doesn't support generic structs, there are several "Result structs": one for each type of value. But each struct follows the same structure as the following example (varying only in the name of the struct and the type of the `value` field):

<table>
<tr><th>TaffyIntResult</th><th>TaffyDimensionResult</th></tr>
<tr>
<td>

```c
typedef struct TaffyIntResult {
  enum TaffyReturnCode return_code;
  int32_t value;
} TaffyIntResult;
```

</td>
<td>

```c
typedef struct TaffyDimensionResult {
  enum TaffyReturnCode return_code;
  struct TaffyDimension value;
} TaffyDimensionResult;
```

</td>
</tr>
</table>

Functions that return Result structs will either return a `TAFFY_RETURN_CODE_OK` along with a meaningful value, or a error variant of `TaffyReturnCode` along with

## API Usage

### Tree Creation and Manipulation

The `TaffyTree` struct is the entrypoint to the Taffy C API and represents an entire tree of UI nodes. Taffy uses arena allocation, so the `TaffyTree` owns the entire tree of nodes at all times.

You only ever get access to a `TaffyNodeId` handle which can be used to manipulate the structure of the tree, or pointers to `TaffyStyle` object (which can be used to set styles on a Node, but are considered borrowed pointers and thus must only be held temporarily).

#### Lifecycle

- `TaffyTree_New` allocates a new `TaffyTree` and returns an owned pointer to it.
- `TaffyTree_Free` can be used to free the tree once you are done with it.

All other functions in the API which accept a pointer to a `TaffyTree` have borrowing semantics: they access the tree during the duration of the function (and if the pointer is not a `const` pointer, may modify the tree), but will not store the pointer or take ownership of the tree.

#### Node creation and manipulation

- `TaffyTree_NewNode` creates a new Node within the tree and returns a `TaffyNodeId` handle to it. The node will initially have the default styles

### Setting styles on node
