using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using UnityEditor.Experimental.GraphView;
using UnityEngine;
using UnityEngine.Serialization;

[System.Serializable]
public struct Layer
{
    public int y;
    public Coord[] blocks;
}

[System.Serializable]
public struct Coord
{
    public int[] b;
}

[System.Serializable]
public struct BlockData
{
    public uint x;
    public uint z;
}

[System.Serializable]
public struct Grouping
{
    public Layer[] layers;
}

[System.Serializable]
public struct CoordsExport
{
    public Grouping[] groupings;
}

[Serializable]
class TextureFNames
{
    public string[] textures;
}

public class MCArr : MonoBehaviour
{
    [Header("Input")] public GameObject parent;
    public string textureFolder;
    public string textureJsonPath;
    public string modelJsonPath;

    [Header("Output")] [Range(0f, 2f)] public float cubeScale = 1f;
    [Range(0f, 5f)] public float cubeSpacing = 1f;

    private CoordsExport model;
    private long prevFileLength;
    private bool newInput;
    private List<Texture2D> textures;

    void Start()
    {
        // load textures from /textures folder
        TextureFNames texture_fnames = JsonUtility.FromJson<TextureFNames>(File.ReadAllText(textureJsonPath));


        textures = new List<Texture2D>();
        foreach (var file in texture_fnames.textures)
        {
            if (file.EndsWith(".meta"))
                continue;
            var texture = new Texture2D(2, 2);
            texture.LoadImage(File.ReadAllBytes($"{textureFolder}/{file}"));
            textures.Add(texture);
        }

        // ensure json gets loaded
        prevFileLength = -1;
    }

    private bool ShouldUpdate()
    {
        var fileInfo = new System.IO.FileInfo(modelJsonPath);
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

        // print($"Before: {before} After: {parent.transform.childCount}");

        // Read file
        {
            string json = "";
            StreamReader inp_stm = new StreamReader(modelJsonPath);
            while (!inp_stm.EndOfStream)
            {
                json += inp_stm.ReadLine();
            }

            inp_stm.Close();

            model = JsonUtility.FromJson<CoordsExport>(json);
        }

        // Create cubes
        {
            for (int group = 0; group < model.groupings.Length; group++)
            {
                var grouping = model.groupings[group].layers;
                for (int layer = 0; layer < grouping.Length; layer++)
                {
                    var layerData = grouping[layer].blocks;

                    for (int i = 0; i < layerData.Length; i++)
                    {
                        var coord = layerData[i];


                        var x = coord.b[0];
                        var z = coord.b[1];

                        GameObject cube = GameObject.CreatePrimitive(PrimitiveType.Cube);
                        cube.transform.parent = parent.transform;
                        cube.transform.localPosition =
                            new Vector3(x * cubeSpacing, layer * cubeSpacing, z * cubeSpacing);
                        cube.transform.localScale = new Vector3(cubeScale, cubeScale, cubeScale);

                        cube.GetComponent<Renderer>().material.mainTexture = textures[group];
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
