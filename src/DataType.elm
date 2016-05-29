module DataType exposing ( Model, Node (..), Msg (..) )

import Dict
import Http

type alias Model = {
    path : List String,
    nodes : Dict.Dict String Node,
    logs : List String
    }

type Node = NodeLoading | NodeContent {content: String, child: List String}

type Msg = EventLeftClick String | EventPathClick Int | GetFromPathErr Http.Error | GetFromPathOk (List String, String)
