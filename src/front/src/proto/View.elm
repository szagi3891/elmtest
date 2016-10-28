module View exposing (view)

import Html exposing (div, button, text, br, textarea, span)
import Html.Attributes exposing (style, class)
import Html.Events exposing (onClick, onInput)

import DataType exposing (..)
import Utils exposing (enumerate)

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
        currentNode = nodeListGet model.nodes model.path
    in
        case currentNode of
            Just node ->
                ( case node of
                    NodeLoading -> "Ładowanie zawartości ..."
                    NodeContent {content, child} -> content )
            Nothing ->
                "Brak noda"


makePathItem (index, name) = span [class "panel_path_item", onClick (EventPathClick index)] [text name]

makeLeftList model = 

    let
        currentNode = nodeListGet model.nodes model.path
    in
        case currentNode of
            Just node ->
                ( case node of
                    NodeLoading -> [span [] [text "Ładowanie zawartości"]]
                    NodeContent {content, child} -> List.map makeLeftListItem ([".."] ++ child) )
            Nothing ->
                [ makeLeftListItem "..", span [] [text "Brak noda"]]


makeLeftListItem name = div [class "left_item", onClick (EventLeftClick name)] [text name]

makeLogItem line = div [class "logs_line"] [text line]

