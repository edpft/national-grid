export const openTab = (tabName) => {
    let tabContent = document.getElementsByClassName("tab-content");
    Array.from(tabContent).forEach(element => {
        if (element.id == tabName) {
            element.classList.remove("inactive");
            element.classList.add("column-centre");
        } else {
            element.classList.remove("column-centre");
            element.classList.add("inactive");
        }
    });
}