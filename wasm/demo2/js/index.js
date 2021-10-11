import("../pkg/index.js")
  .then((wasm) => {
    console.log(wasm) // wasm module
    window.fileImport = function fileImport() {
      //获取读取我文件的 File 对象
      var selectedFile = document.getElementById("file").files[0];
      var reader = new FileReader(); // 这是核心, 读取操作就是由它完成.
      reader.readAsArrayBuffer(selectedFile); // 读取文件的内容,也可以读取文件的URL
      reader.onload = function () {
        var uint8Array = new Uint8Array(this.result);
        wasm.gray_scale(uint8Array);
      };
    };
  })
  .catch(console.error);
