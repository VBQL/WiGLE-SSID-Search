pub fn state_boundaries(query: String) -> [f64;4]{
  match query.as_str(){
    "AK" => [52.5964,71.5232,-169.9146,-129.993],
    "AL" => [30.1463,35.0041,-88.4743,-84.8927],
    "AR" => [33.0075, 36.4997, -94.6198, -89.6594],
    "AZ" => [31.3325, 37.0004, -114.8126, -109.0475],
    "CA" => [32.5121, 42.0126, -124.6509, -114.1315],
    "CO" => [36.9949, 41.0006, -109.0489, -102.0424],
    "CT" => [40.9509, 42.0511, -73.7272, -71.7874],
    "DE" => [38.4482, 39.8296, -75.7919, -74.8526],
    "FL" => [24.3959, 31.0035, -87.6256, -79.8198],
    "GA" => [30.3575, 34.9996, -85.6082, -80.696],
    "HI" => [18.71, 22.3386, -160.3922, -154.6271],
    "IA" => [40.3622, 43.5008, -96.6357, -90.1538],
    "ID" => [41.9871, 49.0018, -117.2372, -111.0471],
    "IL" => [36.9894, 42.5116, -91.512, -87.0213],
    "IN" => [37.7718, 41.7611, -88.098, -84.809],
    "KS" => [36.9927, 40.0087, -102.0506, -94.6046],
    "KY" => [36.4931, 39.1439, -89.5372, -82.0308],
    "LA" => [28.8832, 33.0225, -94.043, -88.7421],
    "MA" => [41.159, 42.889, -73.5081, -69.7398],
    "MD" => [37.8889, 39.722, -79.4861, -74.8581],
    "ME" => [42.9182, 47.455, -71.0829, -66.8628],
    "MI" => [41.6965, 48.3042, -90.4175, -82.1221],
    "MN" => [43.5008, 49.3877, -97.2304, -89.4919],
    "MO" => [35.9958, 40.6181, -95.7527, -89.1005],
    "MS" => [30.0905, 35.0075, -91.6589, -88.0994],
    "MT" => [44.3563, 48.9991, -116.0458, -104.0186],
    "NC" => [33.7666, 36.588, -84.3201, -75.4129],
    "ND" => [45.934, 48.9982, -104.0501, -96.5671],
    "NE" => [39.9992, 43.0006, -104.0543, -95.3091],
    "NH" => [42.6986, 45.3058, -72.5592, -70.5583],
    "NJ" => [38.8472, 41.3593, -75.5708, -73.8885],
    "NM" => [31.3337, 36.9982, -109.0489, -103.0023],
    "NV" => [35.003, 42.0003, -120.0037, -114.0436],
    "NY" => [40.4772, 45.0153, -79.7624, -71.7517],
    "OH" => [38.3761, 42.321, -84.8172, -80.5188],
    "OK" => [33.6386, 37.0015, -103.0064, -94.4357],
    "OR" => [41.9952, 46.2891, -124.7305, -116.4606],
    "PA" => [39.7199, 42.5167, -80.5243, -74.707],
    "RI" => [41.1849, 42.0156, -71.9041, -71.0541],
    "SC" => [32.0453, 35.2075, -83.3588, -78.4836],
    "SD" => [42.4772, 45.9435, -104.0529, -96.438],
    "TN" => [34.9884, 36.6871, -90.3131, -81.6518],
    "TX" => [25.8419, 36.5008, -106.6168, -93.5074],
    "UT" => [36.9982, 41.9993, -114.0504, -109.0462],
    "VA" => [36.5427, 39.4659, -83.6753, -74.9707],
    "VT" => [42.7289, 45.0153, -73.4381, -71.4949],
    "WA" => [45.5439, 49.0027, -124.8679, -116.9165],
    "WI" => [42.4954, 47.31, -92.8564, -86.2523],
    "WV" => [37.1953, 40.6338, -82.6392, -77.731],
    "WY" => [40.9986, 44.9998, -111.0539, -104.0556],
    "AS" => [-14.377579, -14.221549, -170.851479, -170.539742],
    "DC" => [38.79164435, 39.031386, -77.11979522, -76.867218],
    "FM" => [6.673985, 7.202918, 157.651062, 158.595886],
    "GU" => [13.227058, 14.204108, 144.598961, 145.301743],
    "MH" => [1.7575, 17.9578, 157.4121, 175.5395],
    "MP" => [14.834442, 15.300557, 145.530052, 145.836296],
    "PW" => [1.7355, 11.4584, 129.6166, 136.9116],
    "PR" => [17.904834, 18.520551, -67.289886, -65.177765],
    "VI" => [18.302014, 18.751244, -64.861221, -64.26384],
    _ => [92.0,92.0,181.0,181.0],
  }
}
