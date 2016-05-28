var express = require('express');
var app = new express();
var fs = require("fs");

app.use(express.static('./static'));

app.all('/api/get', getContent);
app.all('/api/get*', getContent);

app.listen(8051, function onDevServerListening(err) {
    if (err) {
        throw err;
    } else {
        //gutil.log(gutil.colors.green('Server started: http://localhost:8051'));
        console.info('Server started: http://localhost:8051');
    }
});


function getContent(req, res){

    var path = req.path;

    if (path.substr(0, 8) === "/api/get") {
        path = path.substr(8);
    } else {
        send_error(res, "błędny parametr path");
        return;
    }

    var root = "./data";

    if (path.indexOf(".") >= 0) {
        send_error(res, "niedozwolony znak '.'");
        return;
    }

    fs.readFile(root + path, function(err, content){

        if (err !== null) {

            fs.readdir(root + path, function(err, list){

                if (err !== null) {

                    send_error(res, "brak elementu: " + path);

                } else {

                    res.send(JSON.stringify({
                        "status" : "ok",
                        "content" : "",
                        "child" : list
                    }));
                }
            });

        } else {

            res.send(JSON.stringify({
                "status" : "ok",
                "content" : content.toString(),
                "child" : []
            }));
        }
    });
}

function getFullInfo(root, list, callback) {

    var fullList = list.map(function(item){

        return root + "/" + item;
    })

    var prmisList = [];

    for (var i=0; i<fullList.length; i++) {

        prmisList.push(getPromise(fullList[i]));
    }

    Promise.all(prmisList).then(function(values) {

        var out = {};

        for (var i=0; i<list.length; i++) {
            out[list[i]] = values[i];
        }

        callback(true, out);

    }, function() {
        callback(false);
    });

    function getPromise(path) {
        return new Promise(function(done, reject){
            fs.lstat(path, function(err, stat){

                if (err !== null) {
                    reject(err);
                } else {
                    if (stat.isFile()){
                        done({type : "file"});
                    } else if (stat.isDirectory()) {
                        done({type: "dir"});
                    } else {
                        reject("nieznany typ");
                    }
                }
            });
        });
    }
}

function send_error(res, message) {

    res.send(JSON.stringify({
        "status" : "error",
        "message" : message
    }));
}