var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([102])
             chartData.push([0])
chartData.push([791, 390])
             chartData.push([1, 1])
chartData.push([128, 0, 155, 123, 151, 141, 154, 190, 0, 432, 106, 158, 180, 161, 142, 0, 93, 0, 134, 0, 0, 317, 169, 126, 248, 149, 107, 0, 116, 101, 204])
             chartData.push([20, 50, 16, 26, 11, 23, 20, 9, 50, 1, 28, 21, 14, 15, 19, 50, 49, 50, 38, 50, 50, 0, 24, 39, 6, 26, 41, 50, 47, 41, 2])
chartData.push([189, 0, 419, 372, 236, 395, 296, 191, 170, 218])
             chartData.push([20, 50, 1, 2, 7, 1, 3, 16, 21, 3])
chartData.push([182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 323, 264, 0, 0, 265, 0, 0, 0, 0, 0, 0, 394])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 30, 36, 50, 50, 45, 50, 50, 50, 50, 50, 50, 4])
chartData.push([66, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 70])
             chartData.push([14, 7, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 14, 5])
chartData.push([121, 0, 112, 108, 124, 93, 158, 0, 0, 0, 0, 0, 0, 0, 0, 143, 0, 111, 171, 0, 132, 0, 115, 141, 200, 142, 185])
             chartData.push([25, 50, 35, 33, 33, 46, 19, 50, 50, 50, 50, 50, 50, 50, 50, 36, 50, 45, 29, 50, 41, 50, 44, 23, 11, 27, 6])
chartData.push([128, 0, 173])
             chartData.push([46, 50, 7])
chartData.push([241])
             chartData.push([8])
chartData.push([78, 0, 0, 66, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 109, 0, 0, 0, 102, 0, 0, 0, 0, 127, 78, 102])
             chartData.push([26, 50, 50, 40, 50, 50, 41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 49, 23, 50, 50, 50, 19, 50, 50, 50, 50, 12, 36, 9])
chartData.push([100])
             chartData.push([10])
chartData.push([105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234, 167, 0, 0, 0, 155])
             chartData.push([42, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 19, 50, 50, 50, 11])
chartData.push([139])
             chartData.push([12])
chartData.push([59, 0, 0, 70, 0, 0, 70, 0, 74, 105, 128, 89, 71, 0, 74, 101, 77, 0, 72, 0, 0, 97, 0, 73, 0, 0, 73, 97])
             chartData.push([48, 50, 50, 38, 50, 50, 32, 50, 39, 23, 8, 25, 33, 50, 37, 29, 48, 50, 42, 50, 50, 28, 50, 45, 50, 50, 38, 13])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160, 223, 195, 174, 188, 115, 123, 121, 384, 221, 137])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18, 9, 16, 10, 17, 41, 31, 27, 3, 7, 14])
chartData.push([119])
             chartData.push([15])
chartData.push([213, 160, 0, 158])
             chartData.push([7, 37, 50, 16])
chartData.push([181])
             chartData.push([17])
chartData.push([161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 77])
             chartData.push([7, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 50, 50, 50, 50, 50, 18])
chartData.push([139])
             chartData.push([19])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259, 217, 277, 190, 0, 252, 0, 0, 171, 0, 128, 149])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10, 15, 8, 28, 50, 11, 50, 50, 16, 50, 37, 20])
chartData.push([344, 194, 165, 159, 206, 205, 138, 152, 0, 0, 0, 171, 114, 192, 186, 0, 151, 136, 246, 148, 241, 139, 173, 0, 209, 134, 179, 116, 164, 167, 146])
             chartData.push([4, 14, 20, 22, 7, 11, 27, 22, 50, 50, 50, 25, 49, 16, 17, 50, 25, 28, 14, 39, 11, 34, 33, 50, 21, 45, 20, 43, 34, 25, 21])
chartData.push([489, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 121])
             chartData.push([1, 27, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 22])
chartData.push([69, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 96])
             chartData.push([44, 50, 50, 49, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 44, 50, 23])
chartData.push([107])
             chartData.push([24])
chartData.push([214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 247, 367, 0, 0, 0, 0, 0, 0, 358, 202])
             chartData.push([23, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 25, 13, 50, 50, 50, 50, 50, 50, 9, 25])
chartData.push([216, 0, 177])
             chartData.push([29, 50, 26])
chartData.push([81])
             chartData.push([27])
chartData.push([53, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 56, 73, 92, 60, 69, 79, 0, 65, 0, 115, 56])
             chartData.push([30, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 40, 24, 17, 46, 30, 23, 50, 22, 50, 5, 28])
chartData.push([316, 103, 0, 172, 0, 0, 0, 0, 0, 95, 0, 0, 100, 0, 0, 0, 153, 0, 0, 168, 175, 125, 188, 162, 0, 0, 0, 0, 111])
             chartData.push([2, 31, 50, 10, 50, 50, 50, 50, 50, 48, 50, 50, 39, 50, 50, 50, 26, 50, 50, 21, 21, 40, 18, 20, 50, 50, 50, 50, 29])
chartData.push([196, 955, 556, 170, 143, 168, 0, 144, 161, 0, 0, 0, 0, 205, 197, 121, 143, 111, 122])
             chartData.push([15, 1, 2, 15, 22, 14, 50, 35, 25, 50, 50, 50, 50, 11, 6, 35, 39, 42, 30])
chartData.push([123, 117, 174, 0, 152, 190, 0, 0, 0, 124, 0, 0, 0, 181, 103, 0, 0, 0, 157, 0, 0, 118, 0, 157, 250, 211, 165, 0, 0, 117, 132])
             chartData.push([30, 36, 17, 50, 16, 15, 50, 50, 50, 31, 50, 50, 50, 18, 48, 50, 50, 50, 40, 50, 50, 47, 50, 31, 12, 13, 26, 50, 50, 44, 31])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75, 104, 91, 71, 178, 118, 85, 57, 82, 84, 55])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21, 11, 18, 20, 1, 4, 9, 32, 18, 18, 32])
chartData.push([260, 473, 155, 125, 182, 157, 0, 0, 181, 108])
             chartData.push([4, 2, 15, 37, 12, 15, 50, 50, 16, 33])
chartData.push([325, 298, 141, 280, 176, 216, 123, 126, 206, 188, 217, 108])
             chartData.push([3, 4, 26, 6, 9, 10, 38, 30, 6, 10, 8, 34])
chartData.push([86])
             chartData.push([35])
chartData.push([75, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 99, 88, 0, 0, 96])
             chartData.push([46, 50, 50, 50, 50, 50, 50, 50, 45, 50, 50, 50, 50, 50, 50, 50, 50, 50, 40, 40, 42, 50, 50, 36])
chartData.push([168, 141, 0, 0, 0, 0, 125, 0, 0, 0, 0, 113])
             chartData.push([26, 32, 50, 50, 50, 50, 42, 50, 50, 50, 50, 37])
chartData.push([177, 164, 192, 303, 110])
             chartData.push([10, 11, 15, 2, 38])
chartData.push([70])
             chartData.push([39])
chartData.push([86, 0, 0, 54, 56, 0, 0, 0, 48, 0, 0, 75, 85, 62, 0, 0, 73, 0, 0, 0, 0, 83, 0, 0, 85, 71, 0, 0, 60])
             chartData.push([19, 50, 50, 45, 45, 50, 50, 50, 49, 50, 50, 30, 20, 32, 50, 50, 44, 50, 50, 50, 50, 24, 50, 50, 25, 29, 50, 50, 40])
chartData.push([118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 109, 0, 0, 95])
             chartData.push([25, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 43, 36, 50, 50, 41])
chartData.push([265, 0, 0, 0, 0, 0, 0, 0, 0, 201, 229, 0, 0, 0, 202, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 188])
             chartData.push([23, 50, 50, 50, 50, 50, 50, 50, 50, 44, 35, 50, 50, 50, 42, 50, 50, 50, 30, 50, 50, 50, 50, 50, 50, 50, 50, 42])
chartData.push([56])
             chartData.push([43])
chartData.push([123, 0, 0, 0, 0, 114, 0, 0, 0, 144, 125, 191, 239, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 0, 0, 0, 158, 223, 120, 249, 91])
             chartData.push([21, 50, 50, 50, 50, 29, 50, 50, 50, 14, 24, 13, 8, 50, 50, 50, 50, 50, 50, 50, 50, 50, 27, 50, 50, 50, 13, 5, 42, 3, 44])
chartData.push([48, 0, 0, 0, 0, 0, 104, 95, 0, 0, 0, 0, 99, 188, 188, 0, 104, 0, 0, 55, 54])
             chartData.push([48, 50, 50, 50, 50, 50, 13, 13, 50, 50, 50, 50, 25, 2, 3, 50, 8, 50, 50, 47, 45])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341, 457, 235, 294, 486, 182, 205, 182, 300, 207, 149])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12, 3, 40, 11, 5, 48, 34, 38, 14, 31, 46])
chartData.push([102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96])
             chartData.push([41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 47])
chartData.push([124, 0, 0, 238, 0, 0, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122])
             chartData.push([40, 50, 50, 15, 50, 50, 33, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 48])


for (i = 0; i < charts_number; i++) {
    chartHandles.push(document.getElementById("chart" + i));

    chartSettings.push({
        labels: new Array(chartData[i * 2].length).fill(":)"),
        datasets: [{
            data: chartData[i * 2],
            label: "Scrobbles",
            yAxisID: 'A',
            borderColor: colors[1],
            borderWidth: 4,
            pointBackgroundColor: colors[1],
            backgroundColor: colors[3],
        },
        {
            data: chartData[i * 2 + 1],
            backgroundColor: 'transparent',
            label: "position",
            yAxisID: 'B',
            borderColor: colors[0],
            borderWidth: 4,
            pointBackgroundColor: colors[0]
        }]
    });

    if (chartHandles[i]) {
        new Chart(chartHandles[i], {
            type: 'line',
            data: chartSettings[i],
            options: {
                scales: {
                    yAxes: [{
                        id: 'A',
                        type: 'linear',
                        position: 'left',
                        ticks: {
                            precision: 0,
                        }
                    }, {
                        id: 'B',
                        type: 'linear',
                        position: 'right',
                        ticks: {
                            max: charts_number,
                            min: 1,
                            reverse: true
                        }
                    }]
                },
                legend: {
                    display: false
                }
            }
        });
    }
}