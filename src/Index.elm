import Html exposing (div, button, text, br, textarea, span)
import Html.Attributes exposing (style, class)
import Html.App exposing (program)
import Html.Events exposing (onClick, onInput)
import String
import List
import Http
import Task
import Dict


type alias Model = {
    path : List String,
    nodes : Dict.Dict String Node,
    logs : List String
    }

type Node = NodeLoading | NodeContent {content: String, child: List String}

type Msg = EventLeftClick String | EventPathClick Int | GetFromPathErr Http.Error | GetFromPathOk (List String, String)


main = Html.App.program {
    init = init_model,
    view = view,
    update = update,
    subscriptions = subscriptions
    }


subscriptions : Model -> Sub Msg
subscriptions model = Sub.none

init_model : (Model, Cmd Msg)
init_model = ({ path = [], nodes = Dict.empty, logs = [] }, commandGetFromPath [])


view model =
    div [class "container"] [

        div [class "panel_path"]
            (List.map makePathItem (enumerate (["."] ++ model.path)))

        , div [class "menu"] []
        , div [class "panel_content"] [
            div [class "panel_left"] (makeLeftList model)
            , div [class "panel_right"] [
                button [] [text "Edit"]
            ]
        ]
        , div [class "logs"] (List.map makeLogItem model.logs)
    ]

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
            ({model | logs = model.logs ++ ["odpowiedź z serwera: " ++ message]}, Cmd.none)
        


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
                    