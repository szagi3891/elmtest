import Html exposing (..)
import Html exposing (div, button, text, br, textarea)
import Html.Attributes exposing (style, class)
import Html.App exposing (beginnerProgram)
import Html.Events exposing (onClick, onInput)
import String
import List
import Http
import Task

init_model = { path = ["cos", "dasda"] }

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
            (List.map makePathItem (["."] ++ model.path))

        , div [class "menu"] []
        , div [class "panel_content"] [
            div [class "panel_left"] makeLeftList
            , div [class "panel_right"] [
                button [] [text "Edit"]
            ]
        ]
    ]

makePathItem name = span [class "panel_path_item"] [text name]

makeLeftList = List.map makeLeftListItem ["..", "cosik", "cosik2", "cosik4334"]
makeLeftListItem name = div [class "left_item", onClick (LeftClick name)] [text name]



type Msg = LeftClick String


-- update : Msg Model -> (Model, Cmd Msg)
update msg model =
    case msg of
                                                        -- lewe menu kliknięte
        LeftClick name ->
            ({model | path = (model.path ++ [name])}, Cmd.none)


-- Task.mapError : (x -> y) -> Task x a -> Task y a

