import Html exposing (..)
import Html exposing (div, button, text, br, textarea)
import Html.Attributes exposing (style)
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
  div [] [text "początek"]


type Msg = Show


-- update : Msg Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Show ->
            (model, Cmd.none)



