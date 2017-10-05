module App exposing (..)

import Html exposing (Html, div, text)
import Keyboard

type alias Model =
  { code : Int
  }

type Msg
  = KeyMsg Keyboard.KeyCode

init : ( Model, Cmd Msg )
init = ( { code = 0 } , Cmd.none )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    KeyMsg code ->
      ( { model | code = code }
      , Cmd.none
      )

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.batch
    [ Keyboard.downs KeyMsg
    ]

view : Model -> Html Msg
view model =
  div [] [ text "Hello!" ]

main =
  Html.program
    { init = init
    , update = update
    , view = view
    , subscriptions = subscriptions
    }
