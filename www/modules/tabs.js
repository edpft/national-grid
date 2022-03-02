export const openTab = (tabName) => {
    let tabContent = document.getElementsByClassName("tab-content");
    Array.from(tabContent).forEach(element => {
        if (element.id == tabName) {
            element.classList.remove("inactive");
            element.classList.add("active");
        } else {
            element.classList.remove("active");
            element.classList.add("inactive");
        }
    });
}