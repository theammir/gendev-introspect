<h1 align="center">Arguably better <code>badIntrospect</code> implementation</h1>
</br>

JavaScript                                                                                 |  Output
:-----------------------------------------------------------------------------------------:|:-----------------------------------------------------------------------------------------:
![image](https://github.com/user-attachments/assets/486636c7-3263-4c6c-8323-1caf8b226524)  |  ![image](https://github.com/user-attachments/assets/587e52ab-ded7-4d0c-84d7-3a0619c37ba0)


# ✨ Features
* ## 🦀 Utilizes Rust's type safety
  <sub>we're going to convert these to JS Objects anyway, so, uh, speaking of which,</sub>
* ## 💠 Compiled into a Node.js native addon, powered by [napi](https://napi.rs)
  Just so you can use it on actual JavaScript functions, the ones you were so cruel to convert into plain text.
* ## My code is not _that_ scary

# Usage
The library can be built with
```bash
npm i
npm run build
```
For the sake of example, the binary is moved to `js_project` folder, where `index.js` can be executed.
```bash
cd js_project
node index.js
```
