using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using NativeWebSocket;

public class SocketClient : MonoBehaviour
{
    WebSocket websocket;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    async void Start()
    {
        Debug.Log("Starting...");

        websocket = new WebSocket("ws://127.0.0.1:8000/echo");

        websocket.OnOpen += () =>
        {
            Debug.Log("Connection open!");
        };

        websocket.OnError += (e) =>
        {
            Debug.Log("Error! " + e);
        };

        websocket.OnClose += (e) =>
        {
            Debug.Log("Connection closed!");
        };

        websocket.OnMessage += (bytes) =>
        {
            string result = System.Text.Encoding.UTF8.GetString(bytes);
            Debug.Log("OnMessage!" + result);
        };

        // Keep sending messages at every 0.3s
        // InvokeRepeating("SendWebSocketMessage", 0.0f, 0.3f);

        // waiting for messages
        await websocket.Connect();
    }

    void Update()
    {
      #if !UNITY_WEBGL || UNITY_EDITOR
        websocket.DispatchMessageQueue();
      #endif
    }

    private async void OnApplicationQuit()
    {
      await websocket.Close();
    }
}
