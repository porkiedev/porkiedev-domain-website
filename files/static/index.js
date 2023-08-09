function testfunc() {
    console.log(document.getElementById("main_header_button2").innerHTML)
    fetch("/api/test", {
        method: "POST",
        body: JSON.stringify({
            test: "abc"
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    })
    .then((response) => response.json())
    .then((json) => console.log(json));

    window.location.pathname = "/dietz"
    console.log(window.location.pathname);
}
