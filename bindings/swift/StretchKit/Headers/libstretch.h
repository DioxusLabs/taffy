#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct StretchStyleDimension {
  int32_t dimen_type;
  float dimen_value;
} StretchStyleDimension;

typedef struct StretchStyleRect {
  struct StretchStyleDimension start;
  struct StretchStyleDimension end;
  struct StretchStyleDimension top;
  struct StretchStyleDimension bottom;
} StretchStyleRect;

typedef struct StretchStyleSize {
  struct StretchStyleDimension width;
  struct StretchStyleDimension height;
} StretchStyleSize;

typedef struct StretchSize {
  float width;
  float height;
} StretchSize;

void *stretch_style_create(int32_t display,
                           int32_t position_type,
                           int32_t direction,
                           int32_t flex_direction,
                           int32_t flex_wrap,
                           int32_t overflow,
                           int32_t align_items,
                           int32_t align_self,
                           int32_t align_content,
                           int32_t justify_content,
                           struct StretchStyleRect position,
                           struct StretchStyleRect margin,
                           struct StretchStyleRect padding,
                           struct StretchStyleRect border,
                           float flex_grow,
                           float flex_shrink,
                           struct StretchStyleDimension flex_basis,
                           struct StretchStyleSize size,
                           struct StretchStyleSize min_size,
                           struct StretchStyleSize max_size,
                           float aspect_ratio);

void stretch_style_free(void *style);

void *stretch_init(void);

void stretch_free(void *stretch);

void *stretch_node_create(void *stretch, void *style);

void stretch_node_free(void *stretch, void *node);

void stretch_node_set_measure(void *stretch,
                              void *node,
                              void *swift_ptr,
                              struct StretchSize (*measure)(const void*, float, float));

void stretch_node_set_style(void *stretch, void *node, void *style);

bool stretch_node_dirty(void *stretch, void *node);

void stretch_node_mark_dirty(void *stretch, void *node);

void stretch_node_add_child(void *stretch, void *node, void *child);

void stretch_node_replace_child_at_index(void *stretch, void *node, uintptr_t index, void *child);

void stretch_node_remove_child(void *stretch, void *node, void *child);

void stretch_node_remove_child_at_index(void *stretch, void *node, uintptr_t index);

void *stretch_node_compute_layout(void *stretch,
                                  void *node,
                                  float width,
                                  float height,
                                  void *(*create_layout)(const float*));
