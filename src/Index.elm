import Html exposing (..)
import Html exposing (div, button, text, br, textarea)
import Html.Attributes exposing (style, class)
import Html.App exposing (beginnerProgram)
import Html.Events exposing (onClick, onInput)
import String
import List
import Http
import Task


main = Html.App.program {
    init = ({}, Cmd.none),
    view = view,
    update = update,
    subscriptions = subscriptions
    }

-- subscriptions : Model -> Sub Msg
subscriptions model = Sub.none


view model =
    div [class "container"] [
        div [class "panel_path"] [
            div [class "panel_path_item"] [text "."]
        ]
        , div [class "menu"] []
        , div [class "panel_content"] [
            div [class "panel_left"] []
            , div [class "panel_right"] [
                button [] [text "Edit"]
            ]
        ]
    ]

type Msg = Show


-- update : Msg Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Show ->
            (model, Cmd.none)


-- Task.mapError : (x -> y) -> Task x a -> Task y a

