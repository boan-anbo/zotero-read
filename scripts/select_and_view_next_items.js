function selectNextNumberofItems(range, viewItems) {
    const pane = Zotero.getActiveZoteroPane()

    const items = pane.getSortedItems()
    const selectedItems = pane.getSelectedItems();

    const lastSelectedItem = selectedItems[selectedItems.length - 1]

    const selectedItemId = lastSelectedItem.id;


    const currentIndex = items.findIndex(i => {
        return i.id == selectedItemId
    })
    const nextItems = items.slice(currentIndex + 1, currentIndex + 1 + range)

    if (nextItems) {
        pane.selectItems(nextItems.map(i => i.id))
        if (viewItems) {
            pane.viewItems(nextItems)
        }


    } else {
        return;
    }

}

const range = parseInt(query.range, 10);
const view_items = query.view_items === 'true';

return await selectNextNumberofItems(range, view_items);
