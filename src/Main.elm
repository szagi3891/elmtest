import Html exposing (div, button, text, br, textarea)
import Html.Attributes exposing (style)
import Html.App exposing (beginnerProgram)
import Html.Events exposing (onClick, onInput)
import String
import List

main =
  beginnerProgram { model = {count = 0, message1 = "init", message2 = "init"}, view = view, update = update }

getStyle model = if (model.message1 == model.message2) then "green" else "red"
getText model = if (model.message1 == model.message2) then "ok" else "error"
getLength text = " (" ++ toString (String.length text) ++ ") "

view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (toString model.count) ]
    , button [ onClick Increment ] [ text "+" ]
    , br [] []
    , button [ onClick Add ] [ text "+10" ]
    , br [] []
    , button [ onClick Reset ] [ text "reset" ]
    , br [] []
    , br [] []
    , textarea [onInput Change1] [ text model.message1 ]
    , textarea [onInput Change2] [ text model.message2 ]
    , div [] [ text (String.reverse model.message1) ]
    , div [] [ text (model.message1 ++ (getLength model.message1) ++ " --- " ++ model.message2 ++ (getLength model.message2)) ]
    , div [style [("color", getStyle model)]] [ text (getText model)]
    ]


type Msg = Increment | Decrement | Add | Reset | Change1 String | Change2 String


update msg model =
    case msg of
        Increment ->
            {model | count = model.count + 1}

        Decrement ->
            {model | count = model.count - 1}

        Add ->
            {model | count = model.count + 10}
    
        Reset ->
            {model | count = 0}
        
        Change1 new_content ->
            {model | message1 = new_content}
            
        Change2 new_content ->
            {model | message2 = new_content}