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
    init = ({count = 0, message1 = "init", message2 = "init", resp = "..."}, Cmd.none),
    view = view,
    update = update,
    subscriptions = subscriptions
    }

-- subscriptions : Model -> Sub Msg
subscriptions model = Sub.none
  

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
    , button [onClick Get] [ text "Zrób request" ]
    , button [onClick ResetGet] [ text "reset" ]
    , br [] []
    , textarea [onInput Change3, style [("width", "100%"), ("height", "140px")]] [text model.resp]
    , div
        [style
            [("white-space", "pre-line")
            ,("word-break", "break-all")
            ,("padding", "10px")
            ,("border", "1px solid black")
            ,("margin-top", "10px")
            ]
        ]
        [text model.resp]
    ]


type Msg = Increment | Decrement | Add | Reset | Change1 String | Change2 String
         | Get | Change3 String | ResetGet | GetOk (String, String) | GetErr Http.Error


update msg model =
    case msg of
        Increment ->
            ({model | count = model.count + 1}, Cmd.none)

        Decrement ->
            ({model | count = model.count - 1}, Cmd.none)

        Add ->
            ({model | count = model.count + 10}, Cmd.none)
    
        Reset ->
            ({model | count = 0}, Cmd.none)
        
        Change1 new_content ->
            ({model | message1 = new_content}, Cmd.none)
            
        Change2 new_content ->
            ({model | message2 = new_content}, Cmd.none)
        
        Change3 new_content ->
            ({model | resp = new_content}, Cmd.none)
        
                        -- Task.perform : (x -> msg) -> (a -> msg) -> Task x a -> Cmd msg
                        -- getString : String -> Task Error String      Http.Error
        Get ->
            ({model | resp = "trwa ładowanie"}, commandFromUrl "http://api.giphy.com/v1/gifs/random?api_key=dc6zaTOxFJmzC&tag=cats")
        
        ResetGet ->
            ({model | resp = "..."}, Cmd.none)
        
        GetOk (url, message) ->
            ({model | resp = "url : " ++ url ++ "\ncontent: " ++ message}, Cmd.none)
        
        GetErr _ ->
            (model, Cmd.none)


commandFromUrl url =  Task.perform GetErr GetOk (Task.map (contextUrl url) (Http.getString url))

contextUrl url = \resp -> (url, resp)


