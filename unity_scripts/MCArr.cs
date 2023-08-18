using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using UnityEngine;
using UnityEngine.Serialization;

[Serializable]
class ZAxis
{
    public bool[] b;
}

[Serializable]
class YAxis
{
    public ZAxis[] b;
}

[Serializable]
class Model
{
    public YAxis[] b;

    [NonSerialized] public (int, int, int) dims;

    public bool this[int x, int y, int z]
    {
        get { return b[x].b[y].b[z]; }
    }
}

public class MCArr : MonoBehaviour
{
    [Header("Input")] public GameObject parent;
    public string fileName;

    [Header("Output")] [Range(0f, 2f)] public float cubeScale = 1f;
    [Range(0f, 5f)] public float cubeSpacing = 1f;

    private Model model;
    private long prevFileLength;
    private bool newInput;

    void Start()
    {
        prevFileLength = -1;
    }

    private bool ShouldUpdate()
    {
        var fileInfo = new System.IO.FileInfo(fileName);
        bool differentFile = fileInfo.Length != prevFileLength || newInput;
        newInput = false;
        prevFileLength = fileInfo.Length;
        return differentFile;
    }

    void Update()
    {
        if (!ShouldUpdate())
            return;

        var before = parent.transform.childCount;
        // Clear children of parent
        foreach (Transform child in parent.transform)
        {
            Destroy(child.gameObject);
        }
        print($"Before: {before} After: {parent.transform.childCount}");

        // Read file
        {
            string json = "";
            StreamReader inp_stm = new StreamReader(fileName);
            while (!inp_stm.EndOfStream)
            {
                json += inp_stm.ReadLine();
            }

            inp_stm.Close();


            model = JsonUtility.FromJson<Model>(json);
            model.dims = (
                model.b.Length,
                model.b[0].b.Length,
                model.b[0].b[0].b.Length
            );
        }

        // Create cubes
        {
            for (int x = 0; x < model.dims.Item1; x++)
            {
                for (int y = 0; y < model.dims.Item2; y++)
                {
                    for (int z = 0; z < model.dims.Item3; z++)
                    {
                        if (model[x, y, z])
                        {
                            GameObject cube = GameObject.CreatePrimitive(PrimitiveType.Cube);
                            cube.transform.parent = parent.transform;
                            cube.transform.localPosition =
                                new Vector3(x * cubeSpacing, y * cubeSpacing, z * cubeSpacing);
                            cube.transform.localScale = new Vector3(cubeScale, cubeScale, cubeScale);
                        }
                    }
                }
            }
        }
    }

    public void OnValidate()
    {
        newInput = true;
        Update();
    }
}