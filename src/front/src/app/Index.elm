import Html.App exposing (program)
import DataType exposing (Model)

main = Html.App.program {
    init = init_model,
    view = view,
    update = update
    }

init_model : (Model, Cmd Msg)
init_model = ({path = [], nodes = nodeListInit NodeLoading, logs = [] }, commandGetFromPath [])


type alias Model = {
    path : List String,
    nodes : Dict.Dict String Node,
    logs : List String
    }