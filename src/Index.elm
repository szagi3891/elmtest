import Html exposing (..)
import Html exposing (div, button, text, br, textarea)
import Html.Attributes exposing (style, class)
import Html.App exposing (beginnerProgram)
import Html.Events exposing (onClick, onInput)
import String
import List
import Http
import Task

init_model = { path = ["cos", "dasda", "ddd"] }

main = Html.App.program {
    init = (init_model, Cmd.none),
    view = view,
    update = update,
    subscriptions = subscriptions
    }

-- subscriptions : Model -> Sub Msg
subscriptions model = Sub.none


view model =
    div [class "container"] [

        div [class "panel_path"]
            (List.map makePathItem (enumerate (["."] ++ model.path)))

        , div [class "menu"] []
        , div [class "panel_content"] [
            div [class "panel_left"] makeLeftList
            , div [class "panel_right"] [
                button [] [text "Edit"]
            ]
        ]
    ]

makePathItem (index, name) = span [class "panel_path_item", onClick (EventPathClick index)] [text name]

makeLeftList = List.map makeLeftListItem ["..", "cosik", "cosik2", "cosik4334"]
makeLeftListItem name = div [class "left_item", onClick (EventLeftClick name)] [text name]


-- numeruje listę od 0 do length-1
enumerate x = zip [0..(List.length x) - 1] x

zip : List a -> List b -> List (a,b)
zip = List.map2 (,)


type Msg = EventLeftClick String| EventPathClick Int


-- update : Msg Model -> (Model, Cmd Msg)
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


-- Task.mapError : (x -> y) -> Task x a -> Task y a
