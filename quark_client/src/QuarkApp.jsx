import React, { useState, useRef } from "react";
import PostList from "./PostList";
import "./index.css"

function QuarkApp() {
    const ref = useRef(null);

    const handleClick = () => {
        console.log(ref.current.value);
        fetch("http://127.0.0.1:1234/")
        .then(response => response.json())
        .then(data => {console.log(data.test)});
    };

    return (
        <div className="main">
            <textarea id="post_text" ref={ref} rows="10" cols="100"></textarea>
            <button onClick={handleClick}>Publish</button>
            <PostList/>
        </div>
    )
};

export default QuarkApp;