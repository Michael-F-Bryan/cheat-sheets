JavaScript
==========

Despite being prone to dooming you to callback hell, JavaScript can give you a
lot of power and flexibility when it comes to accessing the browser and
manipulating the DOM.


Jquery
------

`Jquery` is a neat library that wraps some JavaScript's native methods of
accessing and manipulating DOM objects and gives it a sane API. It allows you
to easily access any element just using [CSS Selectors][jquery-selectors], and
lets you bind callbacks to various events with the `.on()` method.

To use `Jquery`, add the following script tag to your page:

    <script src="https://code.jquery.com/jquery-3.1.0.min.js"
    integrity="sha256-cCueBR6CsyA4/9szpPfrX3s49M9vUU5BgtiJj06wt/s="
    crossorigin="anonymous"></script>


Events
------

JavaScript exposes [a lot][events] of events, here are just a few of the most
common ones:

<table class="table">
    <thead>
        <tr>
            <th>Mouse Events</th>
            <th>Keyboard Events</th>
            <th>Form Events</th>
            <th>Document/Window Events</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>click</td>
            <td>keypress</td>
            <td>submit</td>
            <td>load</td>
        </tr>
        <tr>
            <td>dblclick</td>
            <td>keydown</td>
            <td>change</td>
            <td>resize</td>
        </tr>
        <tr>
            <td>mouseenter</td>
            <td>keyup</td>
            <td>focus</td>
            <td>scroll</td>
        </tr>
        <tr>
            <td>mouseleave</td>
            <td></td>
            <td>blur</td>
            <td>unload</td>
        </tr>
    </tbody>
</table>

Binding an action to an event with `Jquery` is really easy. Say you want to pop
up an alert every time someone mouses over an element, you could do:

    #!javascript
    $("#element").on("mouseenter", function(e) {alert("Look at me!")});


`Jquery` also has methods that will bind to a specific event, instead of the
more general `.on()` method. For example, you can do something similar with the
`.hover()` method:

    #!javascript
    $("#element").hover(function(e) {alert("Look at me!")});

A useful pattern for running code once the entire page has loaded is to add a
callback to the `$(document).ready()` event.

Each event handler is passed an object with information about the event. Most
event objects have attributes specific to the event which occurred, but here
are some of the more common ones. For more information, [consult the
documentation][event-object].

- **e.currentTarget** - The current DOM element within the event bubbling phase.
- **e.data** - An optional object of data passed to an event method when the
  current executing handler is bound.
- **e.delegateTarget** - The element where the currently-called jQuery event
  handler was attached.
- **e.isDefaultPrevented()** - Returns whether event.preventDefault() was
  ever called on this event object.
- **e.isImmediatePropagationStopped()** - Returns whether
  e.stopImmediatePropagation() was ever called on this event object.
- **e.isPropagationStopped()** - Returns whether event.stopPropagation() was
  ever called on this event object.
- **e.metaKey** - Indicates whether the META key was pressed when the event
  fired.
- **e.namespace** - The namespace specified when the event was triggered.
- **e.pageX** - The mouse position relative to the left edge of the document.
- **e.pageY** - The mouse position relative to the top edge of the document.
- **e.preventDefault()** - If this method is called, the default action of
  the event will not be triggered.
- **e.relatedTarget** - The other DOM element involved in the event, if any.
- **e.result** - The last value returned by an event handler that was
  triggered by this event, unless the value was undefined.
- **e.stopImmediatePropagation()** - Keeps the rest of the handlers from
  being executed and prevents the event from bubbling up the DOM tree.
- **e.stopPropagation()** - Prevents the event from bubbling up the DOM tree,
  preventing any parent handlers from being notified of the event.
- **e.target** - The DOM element that initiated the event.
- **e.timeStamp** - The difference in milliseconds between the time the
  browser created the event and January 1, 1970.
- **e.type** - Describes the nature of the event.
- **e.which** - For key or mouse events, this property indicates the specific
  key or button that was pressed.


Timers
------

Because JavaScript is an event driven programming language, it is quite easy to
set up a recurring timer which will raise an event (which then triggers your
callback) after a period of time.

The two key methods to use with JavaScript are:

<dl>
    <dt>
        setTimeout(function, milliseconds)
    </dt>
    <dd>
        Executes a function, after waiting a specified number of milliseconds.
    </dd>
    <dt>
        setInterval(function, milliseconds)
    </dt>
    <dd>
        Same as setTimeout(), but repeats the execution of the function
        continuously.
    </dd>
</dl>

For example, this is an (annoying) timer that'll raise an `alert()` every 3
seconds.

    #!javascript
    setInterval(function(){ alert("Hello"); }, 3000);


Asynchronous Requests
---------------------

JavaScript has the ability to send of a http request, and then call a callback
once the response is received. Jquery then gives you a nice wrapper around
this. [Read the docs][jquery-ajax] for more details.

- **jQuery.get():** Load data from the server using a HTTP GET request.
- **jQuery.getJSON():** Load JSON-encoded data from the server using a GET 
  HTTP request.  
- **jQuery.getScript():** Load a JavaScript file from the server using a GET HTTP request, then
  execute it.
- **jQuery.post():** Load data from the server using a HTTP POST request.
- **jQuery.load():** Load data from the server and place the returned HTML into the 
   matched element.


Here is an example of grabbing data from a web API and then printing it to the
screen:

    #!javascript
    $.get("http://ip-api.com/json", function(data) {
        $("#ip-address").html("Your IP address is " + data.query);
    });


Possible Applications
---------------------

Here are just a few ideas for applications (both malicious and benign) for
JavaScript and Jquery.


### A Keylogger ###

Because Jquery gives you access to the `keypressed` event, it's quite easy to
bind an event handler to the body element which listens for `keypressed` and
then logs it somewhere.

The event object given to the handler contains the key pressed (as its ascii
value), as well as information such as whether ctrl, alt, or shift were held
down at the time. You can tie this in with either the mouse position or the
`target` to figure out what the user was looking at when they entered some
text.


### Form Validator ###

Similar to the last example, you can hook into a form's `submit` event to then
get whatever was entered into the form. Then using the `$("form").serialize()`
method, you can get this data in a easy to read format and do whatever you want
with it to validate a user's input before sending the form on to wherever it is
meant to go.

It goes without saying that if someone were to compromise a website and inject
their own javascript onto the page, they would also get access to the form
input as well.



[jquery-selectors]: http://www.w3schools.com/jquery/jquery_ref_selectors.asp
[events]: http://www.w3schools.com/jsref/dom_obj_event.asp
[event-object]: https://api.jquery.com/category/events/event-object/
[query-ajax]: https://api.jquery.com/category/ajax/shorthand-methods/
