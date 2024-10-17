/*
    // Send POST data
    https://stackoverflow.com/questions/6396101/pure-javascript-send-post-data-without-a-form

    // Javascript Prototype
    https://www.programiz.com/javascript/prototype

    JavaScript browser objects...(https://www.w3schools.com/js/js_ex_browser.asp)
    - Window objet
    - Screen object
    - Location object

 */

/*let data = { available_height: screen.availHeight.toString(),
             available_width: screen.availWidth.toString(), 
             color_depth: screen.colorDepth.toString(),
             pixel_depth: screen.pixelDepth.toString(),
             height: screen.height.toString(),
             width: screen.width.toString(),                  };*/

let data = `available_height=${screen.availHeight}&available_width:=${screen.availWidth}&color_depth=${screen.colorDepth}&pixel_depth=${screen.pixelDepth}&height=${screen.height}&width=${screen.width}`;             

fetch("/display.html", {

    method: "POST",
    /*headers: {'Content-Type': 'application/json'},*/
    headers: { 'Content-Type': 'text/plain' },
    body: JSON.stringify(data),
    body: data
}).then(res => {

    console.log("Request complete! response:", res);
    
    window.location.replace("http://192.168.100.17:3968/play.html");
});
