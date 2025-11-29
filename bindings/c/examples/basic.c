#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <stdio.h>

#include "taffy.h"

int main() {
    // Create tree
    TaffyTree *tree = TaffyTree_New();

    // Create child (+set styles)
    TaffyNodeId child = TaffyTree_NewNode(tree).value;
    TaffyStyle *child_style = TaffyTree_GetStyleMut(tree, child).value;
    TaffyStyle_SetWidth(child_style, 0.5, TAFFY_UNIT_PERCENT);
    TaffyStyle_SetHeight(child_style, 0, TAFFY_UNIT_AUTO);

    // Create parent (+set styles)
    TaffyNodeId parent = TaffyTree_NewNode(tree).value;
    TaffyStyle *parent_style = TaffyTree_GetStyleMut(tree, parent).value;
    TaffyStyle_SetWidth(parent_style, 100, TAFFY_UNIT_LENGTH);
    TaffyStyle_SetHeight(parent_style, 100, TAFFY_UNIT_LENGTH);
    TaffyStyle_SetJustifyContent(parent_style, TAFFY_ALIGN_CONTENT_CENTER);

    // Setup parent-child relationship
    TaffyTree_AppendChild(tree, parent, child);

    // Compute layout (100x100 viewport)
    printf("\nCompute layout with 100x100 viewport:\n");
    TaffyTree_ComputeLayout(tree, parent, 100, 100);
    TaffyTree_PrintTree(tree, parent);

    // Compute layout (infinite viewport)
    printf("\nCompute layout with infinite viewport:\n");
    TaffyTree_ComputeLayout(tree, parent, INFINITY, INFINITY);
    TaffyTree_PrintTree(tree, parent);

    // Free tree
    TaffyTree_Free(tree);
    return 0;
}
