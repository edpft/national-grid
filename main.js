import { openTab } from "./modules/tabs.js"

let tablinks = document.getElementsByClassName("tab-links");
Array.from(tablinks).forEach(element => {
    element.addEventListener("click", (event) => {
        event.preventDefault();
        openTab(event.target.dataset.tabContent)
    })
});

import init, { reference_to_coordinates, coordinates_to_reference } from "./modules/national_grid.js"
init()
    .then(() => {
        let reference_input = document.getElementById("reference-input");
        let eastings_output = document.getElementById("eastings-output");
        let northings_output = document.getElementById("northings-output");
        function updateCoordinatesOutput(event) {
            let coordinates = reference_to_coordinates(reference_input.value);
            eastings_output.value = coordinates.eastings;
            northings_output.value = coordinates.northings;
            event.preventDefault();
        }
        let submitReference = document.getElementById("submit-reference");
        submitReference.addEventListener("click", updateCoordinatesOutput)

        let eastings_input = document.getElementById("eastings-input");
        let northings_input = document.getElementById("northings-input");
        let reference_output = document.getElementById("reference-output");
        function updateReferenceOutput(event) {
            let reference = coordinates_to_reference(eastings_input.value, northings_input.value);
            reference_output.value = reference;
            event.preventDefault();
        }
        let submitCoordinates = document.getElementById("submit-coordinates");
        submitCoordinates.addEventListener("click", updateReferenceOutput)

    })