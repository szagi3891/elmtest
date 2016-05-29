import Html.App exposing (program)
import String
import List
import Http
import Task
import Json.Decode
import Json.Decode as Json exposing ((:=))
import Platform.Cmd

import DataType exposing (..)
import View exposing (view)


main = Html.App.program {
    init = init_model,
    view = view,
    update = update,
    subscriptions = subscriptions
    }


subscriptions : Model -> Sub Msg
subscriptions model = Sub.none

init_model : (Model, Cmd Msg)
init_model = ({path = [], nodes = nodeListInit NodeLoading, logs = [] }, commandGetFromPath [])
        




update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
    
                                    -- katalog w górę
        EventLeftClick ".." ->
            afterUpdate ({model | path = (List.take (List.length model.path - 1) model.path)}, Cmd.none)
        
                                    -- lewe menu kliknięte
        EventLeftClick name ->
            afterUpdate ({model | path = (model.path ++ [name])}, Cmd.none)

                                    -- przsłączenie ścieżki, na element który został kliknięty
        EventPathClick index ->
            afterUpdate ({model | path = List.take index model.path}, Cmd.none)

        GetFromPathErr error ->
            afterUpdate ({model | logs = model.logs ++ ["problem z odpowiedzią http: " ++ (errorToString error)]}, Cmd.none)
        
        GetFromPathOk (path, message) ->
            let 
                nowy_nod = parseOk message
                new_nodes = nodeListSet model.nodes model.path nowy_nod
                new_logs = model.logs ++ ["odpowiedź z serwera: " ++ (String.join "/" path) ++ " -> " ++ message]
            in
                afterUpdate ({model | logs = new_logs, nodes = new_nodes}, Cmd.none)



afterUpdate (model, cmd) =
    let
        currentNode = nodeListGet model.nodes model.path
    in
        case currentNode of
            Just node -> (
                case node of
                    NodeLoading -> (model, cmd)
                    NodeContent {content, child} -> (
                        let
                            (model, cmdList) = initChild model child
                        in
                            (model, Platform.Cmd.batch ([cmd] ++ cmdList))
                    )
                )
            Nothing ->
                (model, cmd)


initChild : Model -> List String -> (Model, List ( Cmd Msg ))
initChild model child = 
    let
        initChildItem childName (model, cmd) =
            let
                pathChild = model.path ++ [childName]
                
                (nodes, new_cmd) = case (nodeListGet model.nodes pathChild) of
                    Just _ -> (model.nodes, Cmd.none)
                    Nothing -> (nodeListSet model.nodes pathChild NodeLoading, commandGetFromPath pathChild)
            in
                ({model | nodes = nodes}, cmd ++ [new_cmd])
    in
        List.foldr initChildItem (model, []) child



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


parseOk: String -> Node
parseOk message = case (Json.decodeString parseResponseGetOk message) of
    Ok objResp -> NodeContent {content = objResp.content, child = objResp.child}
    Err _ -> NodeContent {content = ".. coś coś coś ..", child = ["dsadas", "dsa", "21"]}
        

-- parseOk message = NodeContent {content = ".. coś coś coś ..", child = ["dsadas", "dsa", "21"]}


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
                    