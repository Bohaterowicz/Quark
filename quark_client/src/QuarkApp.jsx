import React, { useState, useRef } from "react";
import PostList from "./PostList";
import "./index.css"

async function PostQuarkPost(text_value) {
    const res = await fetch("http://127.0.0.1:1234/posts", {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'origin',
        body: JSON.stringify({ name: "AdminTest", text: text_value})
    });

    return res.ok
}

function QuarkApp() {
    const text_ref = useRef(null);

    const handleClick = () => {
        console.log(text_ref.current.value);
        PostQuarkPost(text_ref.current.value).then(data => {
            console.log(data)
            window.location.reload(false);
        })
    };

    return (
        <div className="main">
            <textarea id="post_text" ref={text_ref} rows="10" cols="100"></textarea>
            <button onClick={handleClick}>Publish</button>
            <PostList/>
        </div>
    )
};

export default QuarkApp;