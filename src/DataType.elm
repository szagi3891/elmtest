module DataType exposing (
    Model,
    Msg (..),
    Node (..),
    nodeListInit,
    nodeListGet,
    nodeListSet
    )

import Dict
import Http
import String


type alias Model = {
    path : List String,
    nodes : Dict.Dict String Node,
    logs : List String
    }

type Node = NodeLoading | NodeContent {content: String, child: List String}



type Msg = EventLeftClick String | EventPathClick Int | GetFromPathErr Http.Error | GetFromPathOk (List String, String)


nodeListInit : Node -> Dict.Dict String Node
nodeListInit rootNode = Dict.insert (makeDictPath []) rootNode Dict.empty

nodeListGet nodeList path = Dict.get (makeDictPath path) nodeList

nodeListSet nodeList path node = Dict.insert (makeDictPath path) node nodeList


makeDictPath : List String -> String
makeDictPath path = String.join "/" (["."] ++ path)

