import Html exposing (div, button, text)
import Html.App exposing (beginnerProgram)
import Html.Events exposing (onClick)


main =
  beginnerProgram { model = 0, view = view, update = update }


view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (toString model) ]
    , button [ onClick Increment ] [ text "+" ]
    , button [ onClick Add ] [ text "+10" ]
    ]


type Msg = Increment | Decrement | Add


update msg model =
  case msg of
    Increment ->
      model + 1

    Decrement ->
      model - 1

    Add ->
      model + 10