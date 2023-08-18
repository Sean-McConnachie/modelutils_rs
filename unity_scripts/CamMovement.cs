using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;
using UnityEngine.Serialization;

public class CamMovement : MonoBehaviour
{
    private static bool _mouseLock = false;

    public Vector3 playerVel;
    public Quaternion originalRotation;

    [Header("Maximum Velocity")] [Range(0f, 10f)]
    public float velScale = 5.0f;

    [Header("Camera Sensitivity")] [Range(0f, 2f)]
    public float camSens = 1f;

    [Header("Movement energy loss")] [Range(0f, 10f)]
    public float energyLoss = 1f;

    private float _mX = 0f;
    private float _mY = 0f;

    [MenuItem("Shortcuts/(un)lock mouse %g")]
    private static void ToggleMouse()
    {
        _mouseLock = !_mouseLock;
    }

    void Start()
    {
        playerVel = Vector3.zero;
        originalRotation = transform.localRotation;
    }

    private void ApplyRotation()
    {
        if (_mouseLock)
            return;

        _mX += Input.GetAxis("Mouse X") * camSens;
        _mY += Input.GetAxis("Mouse Y") * camSens;

        Quaternion mYQt = Quaternion.AngleAxis(_mY, Vector3.left);
        Quaternion mXQt = Quaternion.AngleAxis(_mX, Vector3.up);

        transform.localRotation = originalRotation * mXQt * mYQt;
    }

    private void ApplyMovement()
    {
        float fTime = Time.deltaTime;
        Quaternion camDir = transform.localRotation;

        playerVel -= playerVel * (energyLoss * 0.1f);

        Vector3 deltaMov = Vector3.zero;
        if (Input.GetKey(KeyCode.W))
        {
            deltaMov += (camDir * Vector3.forward);
        }
        if (Input.GetKey(KeyCode.S))
        {
            deltaMov += (camDir * Vector3.back);
        }
        if (Input.GetKey(KeyCode.A))
        {
            deltaMov += (camDir * Vector3.left);
        }
        if (Input.GetKey(KeyCode.D))
        {
            deltaMov += (camDir * Vector3.right);
        }

        float vScale = Input.GetKey(KeyCode.LeftShift) ? velScale * 3.0f : velScale;

        playerVel += deltaMov;
        transform.position += (playerVel) * (fTime * vScale);
    }

    void Update()
    {
        ApplyRotation();
        ApplyMovement();
    }
}