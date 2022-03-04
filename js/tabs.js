export const openTab = (tabName) => {
    let tabs = document.getElementsByClassName("tab");
    Array.from(tabs).forEach(element => {
        if (element.id == tabName) {
            element.classList.add("tab-selected");
            element.classList.remove("tab-deselected");
        } else {
            element.classList.remove("tab-selected");
            element.classList.add("tab-deselected");
        }
    });

    let tabContent = document.getElementsByClassName("tab-content");
    Array.from(tabContent).forEach(element => {
        if (element.id == tabName) {
            element.classList.remove("inactive");
        } else {
            element.classList.add("inactive");
        }
    });
}
