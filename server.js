var express = require('express');
var app = new express();

app.use(express.static('./static'));

app.listen(8051, function onDevServerListening(err) {
    if (err) {
        throw err;
    } else {
        //gutil.log(gutil.colors.green('Server started: http://localhost:8051'));
        console.info('Server started: http://localhost:8051');
    }
});