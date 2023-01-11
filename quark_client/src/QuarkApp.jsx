import React, { useState, useEffect, useRef } from "react";
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
        body: JSON.stringify({ username: "AdminTest", post_content: text_value, post_attachments: ""})
    });

    return res.ok
}

async function GetQuarkPosts(loaded_posts_count, new_posts_to_get, most_recent_id=null) {
    let posts = [];

    let most_recent_id_str = ""
    if (most_recent_id != null) {
        most_recent_id_str = "&most_recent_id=" + most_recent_id
    }

    let rest_api_url = "http://127.0.0.1:1234/posts?" + 
                        ("current_post_count=" + loaded_posts_count) +
                        ("&new_post_request_count=" + new_posts_to_get) +
                        most_recent_id_str  

    await fetch(rest_api_url)
    .then(response => response.json())
    .then(data => { 
        const userPosts = data.posts
        for(let i = 0; i < userPosts.length; i++) {
            const post = {id: userPosts[i].id, username: userPosts[i].username, post_content: userPosts[i].post_content}
            posts.push(post);
        }
    });

    return posts
}

function QuarkApp() {
    const text_ref = useRef(null);
    const [postList, setPostList] = useState([])
    
    const handleClick = () => {
        PostQuarkPost(text_ref.current.value).then(data => {
            console.log(data)
            if(data == true) {
                const post = {username: "AdminTest", post_content: text_ref.current.value}
                setPostList([post, ...postList])
                text_ref.current.value = ""
            }
        })
    }

    const handleScroll = () => {
        const winHeight = window.innerHeight;
        const srcHeight = document.documentElement.scrollHeight;
        const scrollTop =
        document.body.scrollTop || document.documentElement.scrollTop;
        const toBottom = srcHeight - winHeight - scrollTop;

        if (toBottom <= 100) {
            let most_recent_id = postList[0].id
            let loaded_posts_count = postList.length
            const new_posts_to_get = 20;
            GetQuarkPosts(loaded_posts_count, new_posts_to_get, most_recent_id).then(new_posts => {
                setPostList([...postList, ...new_posts])
            });
        }
    }

    useEffect(() => {
        window.addEventListener("scroll", handleScroll);

        return () => {
            window.removeEventListener("scroll", handleScroll);
        };
    });
    
    if(postList.length == 0){
        GetQuarkPosts(postList.length, 20).then(posts => {
            setPostList(posts)
        })
    }


    return (
        <div className="main" onScroll={handleScroll}>
            <textarea id="post_text" ref={text_ref} rows="10" cols="100"></textarea>
            <button onClick={handleClick} >Publish</button>
            <PostList posts={postList}/>
        </div>
    )
};

export default QuarkApp;