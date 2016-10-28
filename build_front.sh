cd src/front

elm-make ./src/test/Test.elm --output ./build/test.elm.js
elm-make ./src/proto/Index.elm --output ./build/proto.elm.js