import Html exposing (div, button, text, br, textarea, span)
import Html.Attributes exposing (style, class)
import Html.App exposing (program)
import Html.Events exposing (onClick, onInput)
import String
import List
import Http
import Task
import Dict
import Json.Decode
import Json.Decode as Json exposing ((:=))


type alias Model = {
    path : List String,
    nodes : Dict.Dict String Node,
    logs : List String
    }

type Node = NodeLoading | NodeContent {content: String, child: List String}

type Msg = EventLeftClick String | EventPathClick Int | GetFromPathErr Http.Error | GetFromPathOk (List String, String)

type alias ResponseGetOk = {
    status : String,
    content : String,
    child : List String
    }

parseResponseGetOk : Json.Decode.Decoder ResponseGetOk
parseResponseGetOk = 
    Json.Decode.object3 ResponseGetOk
      ("status" := Json.Decode.string)
      ("content" := Json.Decode.string)
      ("child" := Json.Decode.list Json.Decode.string)


main = Html.App.program {
    init = init_model,
    view = view,
    update = update,
    subscriptions = subscriptions
    }


subscriptions : Model -> Sub Msg
subscriptions model = Sub.none

init_model : (Model, Cmd Msg)
init_model =
    let
        rootPath = makeDictPath []
        rootNode = NodeLoading 
    in
        ({path = [], nodes = Dict.insert rootPath rootNode Dict.empty, logs = [] }, commandGetFromPath [])


makeDictPath : List String -> String
makeDictPath path = String.join "/" (["."] ++ path)


view model =
    div [class "container"] [

        div [class "panel_path"]
            (List.map makePathItem (enumerate (["."] ++ model.path)))

        , div [class "menu"] []
        , div [class "panel_content"] [
            div [class "panel_left"] (makeLeftList model)
            , div [class "panel_right"] [
                button [] [text "Edit"]
                , div [] [text (getContentCurrentNode model)]
            ]
        ]
        , div [class "logs"] (List.map makeLogItem model.logs)
    ]


getContentCurrentNode : Model -> String
getContentCurrentNode model =
    let
        currentNode = Dict.get (makeDictPath model.path) model.nodes
    in
        case currentNode of
            Just node ->
                ( case node of
                    NodeLoading -> "Ładowanie zawartości ..."
                    NodeContent {content, child} -> content )
            Nothing ->
                "Brak noda"


makePathItem (index, name) = span [class "panel_path_item", onClick (EventPathClick index)] [text name]

-- makeLeftList model = List.map makeLeftListItem ["..", "cosik", "cosik2", "cosik4334"]
makeLeftList model = List.map makeLeftListItem ["..", "cosik", "cosik2", "cosik4334"]
makeLeftListItem name = div [class "left_item", onClick (EventLeftClick name)] [text name]

makeLogItem line = div [class "logs_line"] [text line]


-- numeruje listę od 0 do length-1
enumerate : List a -> List (Int, a)
enumerate x = zip [0..(List.length x) - 1] x

zip : List a -> List b -> List (a,b)
zip = List.map2 (,)



update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
    
                                    -- katalog w górę
        EventLeftClick ".." ->
            ({model | path = (List.take (List.length model.path - 1) model.path)}, Cmd.none)
        
                                    -- lewe menu kliknięte
        EventLeftClick name ->
            ({model | path = (model.path ++ [name])}, Cmd.none)

                                    -- przsłączenie ścieżki, na element który został kliknięty
        EventPathClick index ->
            ({model | path = List.take index model.path}, Cmd.none)

        GetFromPathErr error ->
            ({model | logs = model.logs ++ ["problem z odpowiedzią http: " ++ (errorToString error)]}, Cmd.none)
        
        GetFromPathOk (path, message) ->
            let 
                nowy_nod = parseOk message
                new_nodes = Dict.insert (makeDictPath model.path) nowy_nod model.nodes
                new_logs = model.logs ++ ["odpowiedź z serwera: " ++ message]
            in
                ({model | logs = new_logs, nodes = new_nodes}, Cmd.none)


parseOk: String -> Node
parseOk message = NodeContent {content = ".. coś coś coś ..", child = ["dsadas", "dsa", "21"]}
--    case Json.decodeString parseResponseGetOk message of
--        Just objResp -> NodeContent {content = objResp.content, child = objResp.child}
--        Nothing -> NodeContent {content = ".. coś coś coś ..", child = ["dsadas", "dsa", "21"]}
        


-- addLog : Model -> String -> Model
-- addLog model message = model.logs ++ [message]

commandGetFromPath : List String -> Cmd Msg
commandGetFromPath path =  Task.perform GetFromPathErr GetFromPathOk (Task.map (contextUrl path) (Http.getString (commandGetMakeUrl path)))

commandGetMakeUrl : List String -> String
commandGetMakeUrl path = String.join "/" (["/api/get"] ++ path)

contextUrl : List String -> (String -> (List String, String))
contextUrl url = \resp -> (url, resp)

errorToString : Http.Error -> String
errorToString e = case e of
    Http.Timeout -> "timeout"
    Http.NetworkError -> "network error"
    Http.UnexpectedPayload s -> "unexpected payload - " ++ s
    Http.BadResponse c s -> "bad response - " ++ (toString c) ++ " - " ++ s 
                    