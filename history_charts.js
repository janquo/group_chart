var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([684])
             chartData.push([0])
chartData.push([528])
             chartData.push([1])
chartData.push([260, 473])
             chartData.push([4, 2])
chartData.push([124, 0, 0, 0, 0, 0, 0, 0, 0, 125, 0, 65, 0, 0, 53, 0, 0, 65, 207, 0, 121, 154, 278])
             chartData.push([9, 50, 50, 50, 50, 50, 50, 50, 50, 7, 50, 37, 50, 50, 45, 50, 50, 33, 3, 50, 10, 5, 3])
chartData.push([702])
             chartData.push([4])
chartData.push([293])
             chartData.push([5])
chartData.push([325, 298, 141, 280])
             chartData.push([3, 4, 26, 6])
chartData.push([301])
             chartData.push([7])
chartData.push([319, 193, 124, 188, 138, 115, 129, 96, 0, 247, 0, 0, 0, 169, 112, 140, 167, 265, 171, 187, 496, 231, 254])
             chartData.push([3, 9, 27, 10, 12, 28, 22, 34, 50, 4, 50, 50, 50, 12, 29, 20, 15, 4, 19, 16, 2, 7, 8])
chartData.push([195, 241])
             chartData.push([16, 9])
chartData.push([2212, 376, 378])
             chartData.push([0, 6, 10])
chartData.push([174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103, 0, 228])
             chartData.push([17, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 48, 50, 11])
chartData.push([134, 216, 226, 313, 105, 220, 101, 134, 91, 193, 148, 213, 122, 93, 87, 0, 0, 0, 0, 0, 0, 221])
             chartData.push([20, 8, 6, 2, 35, 7, 30, 20, 40, 12, 24, 11, 32, 44, 47, 50, 50, 50, 50, 50, 50, 12])
chartData.push([214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 247, 367])
             chartData.push([23, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 25, 13])
chartData.push([1447, 308, 224])
             chartData.push([1, 1, 14])
chartData.push([161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 323])
             chartData.push([36, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 15])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160, 223, 195])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18, 9, 16])
chartData.push([53, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 56, 73, 92])
             chartData.push([30, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 40, 24, 17])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75, 104, 91])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21, 11, 18])
chartData.push([198, 194])
             chartData.push([17, 19])
chartData.push([90, 172, 0, 0, 0, 0, 0, 127, 0, 324, 227, 0, 0, 193])
             chartData.push([49, 16, 50, 50, 50, 50, 50, 33, 50, 5, 11, 50, 50, 20])
chartData.push([316, 103, 0, 172, 0, 0, 0, 0, 0, 95, 0, 0, 100, 0, 0, 0, 153, 0, 0, 168, 175])
             chartData.push([2, 31, 50, 10, 50, 50, 50, 50, 50, 48, 50, 50, 39, 50, 50, 50, 26, 50, 50, 21, 21])
chartData.push([1150, 267, 161, 155, 0, 254, 224, 265, 278])
             chartData.push([1, 10, 41, 49, 50, 21, 27, 22, 22])
chartData.push([149, 0, 0, 0, 0, 0, 0, 190, 201, 370, 257, 0, 0, 232, 398, 0, 197, 290, 309, 262, 258])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 22, 21, 6, 16, 50, 50, 14, 4, 50, 39, 13, 9, 18, 23])
chartData.push([128, 0, 155, 123, 151, 141, 154, 190, 0, 432, 106, 158, 180, 161, 142, 0, 93, 0, 134, 0, 0, 317, 169])
             chartData.push([20, 50, 16, 26, 11, 23, 20, 9, 50, 1, 28, 21, 14, 15, 19, 50, 49, 50, 38, 50, 50, 0, 24])
chartData.push([48, 0, 0, 0, 0, 0, 104, 95, 0, 0, 0, 0, 99])
             chartData.push([48, 50, 50, 50, 50, 50, 13, 13, 50, 50, 50, 50, 25])
chartData.push([153, 0, 135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224])
             chartData.push([26, 50, 36, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26])
chartData.push([123, 0, 0, 0, 0, 114, 0, 0, 0, 144, 125, 191, 239, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159])
             chartData.push([21, 50, 50, 50, 50, 29, 50, 50, 50, 14, 24, 13, 8, 50, 50, 50, 50, 50, 50, 50, 50, 50, 27])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259, 217, 277, 190])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10, 15, 8, 28])
chartData.push([121, 0, 112, 108, 124, 93, 158, 0, 0, 0, 0, 0, 0, 0, 0, 143, 0, 111, 171])
             chartData.push([25, 50, 35, 33, 33, 46, 19, 50, 50, 50, 50, 50, 50, 50, 50, 36, 50, 45, 29])
chartData.push([129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 139])
             chartData.push([17, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 30])
chartData.push([157, 213, 280, 146, 120, 181, 214, 192, 132, 209, 0, 0, 130, 139, 106, 85, 0, 107, 129, 0, 0, 157, 151])
             chartData.push([14, 7, 4, 20, 17, 9, 8, 8, 23, 6, 50, 50, 32, 23, 33, 49, 50, 35, 41, 50, 50, 23, 31])
chartData.push([315, 276, 244, 180])
             chartData.push([7, 7, 13, 32])
chartData.push([344, 194, 165, 159, 206, 205, 138, 152, 0, 0, 0, 171, 114, 192, 186, 0, 151, 136, 246, 148, 241, 139, 173])
             chartData.push([4, 14, 20, 22, 7, 11, 27, 22, 50, 50, 50, 25, 49, 16, 17, 50, 25, 28, 14, 39, 11, 34, 33])
chartData.push([140, 0, 0, 0, 174, 307, 157])
             chartData.push([24, 50, 50, 50, 19, 2, 34])
chartData.push([265, 171])
             chartData.push([10, 35])
chartData.push([155])
             chartData.push([36])
chartData.push([122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 155])
             chartData.push([28, 50, 50, 50, 50, 50, 50, 50, 50, 50, 44, 33, 50, 50, 50, 50, 50, 50, 50, 50, 50, 41, 37])
chartData.push([169])
             chartData.push([38])
chartData.push([112])
             chartData.push([39])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341, 457, 235])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12, 3, 40])
chartData.push([166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 190, 256, 178])
             chartData.push([20, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26, 15, 41])
chartData.push([163])
             chartData.push([42])
chartData.push([224])
             chartData.push([43])
chartData.push([113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 280, 0, 0, 184, 176, 135, 157])
             chartData.push([37, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 7, 50, 50, 24, 24, 35, 44])
chartData.push([87])
             chartData.push([45])
chartData.push([160])
             chartData.push([46])
chartData.push([38, 0, 35, 0, 0, 73, 103, 173, 0, 0, 55, 0, 128, 0, 0, 49])
             chartData.push([35, 50, 41, 50, 50, 13, 4, 3, 50, 50, 19, 50, 4, 50, 50, 47])
chartData.push([125, 144, 217, 0, 222, 148, 146, 0, 0, 0, 130, 169, 160, 0, 112, 0, 0, 0, 0, 0, 0, 157])
             chartData.push([37, 33, 14, 50, 12, 28, 25, 50, 50, 50, 43, 30, 31, 50, 48, 50, 50, 50, 50, 50, 50, 48])


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