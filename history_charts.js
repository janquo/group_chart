var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([684, 883])
             chartData.push([0, 0])
chartData.push([189, 0, 419])
             chartData.push([20, 50, 1])
chartData.push([48, 0, 0, 0, 0, 0, 104, 95, 0, 0, 0, 0, 99, 188])
             chartData.push([48, 50, 50, 50, 50, 50, 13, 13, 50, 50, 50, 50, 25, 2])
chartData.push([140, 0, 0, 0, 174, 307, 157, 260])
             chartData.push([24, 50, 50, 50, 19, 2, 34, 3])
chartData.push([528, 206])
             chartData.push([1, 4])
chartData.push([94, 322, 0, 0, 0, 115, 0, 0, 254, 203, 0, 143, 112, 0, 134, 0, 0, 0, 0, 0, 0, 0, 220])
             chartData.push([45, 3, 50, 50, 50, 34, 50, 50, 5, 13, 50, 31, 41, 50, 26, 50, 50, 50, 50, 50, 50, 50, 5])
chartData.push([702, 330])
             chartData.push([4, 6])
chartData.push([1447, 308, 224, 209])
             chartData.push([1, 1, 14, 7])
chartData.push([107])
             chartData.push([8])
chartData.push([325, 298, 141, 280, 176])
             chartData.push([3, 4, 26, 6, 9])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160, 223, 195, 174])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18, 9, 16, 10])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341, 457, 235, 294])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12, 3, 40, 11])
chartData.push([163])
             chartData.push([12])
chartData.push([319, 193, 124, 188, 138, 115, 129, 96, 0, 247, 0, 0, 0, 169, 112, 140, 167, 265, 171, 187, 496, 231, 254, 158])
             chartData.push([3, 9, 27, 10, 12, 28, 22, 34, 50, 4, 50, 50, 50, 12, 29, 20, 15, 4, 19, 16, 2, 7, 8, 13])
chartData.push([170, 149, 104, 0, 0, 0, 0, 0, 132, 173, 0, 0, 0, 0, 0, 186])
             chartData.push([18, 24, 41, 50, 50, 50, 50, 50, 35, 16, 50, 50, 50, 50, 50, 14])
chartData.push([260, 473, 155])
             chartData.push([4, 2, 15])
chartData.push([169])
             chartData.push([16])
chartData.push([298, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168])
             chartData.push([3, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 17])
chartData.push([2103, 749, 297, 419, 300, 215, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 264])
             chartData.push([1, 1, 14, 6, 11, 29, 50, 20, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 18])
chartData.push([78, 0, 0, 66, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 109, 0, 0, 0, 102])
             chartData.push([26, 50, 50, 40, 50, 50, 41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 49, 23, 50, 50, 50, 19])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75, 104, 91, 71])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21, 11, 18, 20])
chartData.push([840, 397, 230, 233, 0, 0, 0, 182])
             chartData.push([2, 2, 17, 18, 50, 50, 50, 21])
chartData.push([106, 116, 176, 126, 0, 115, 0, 0, 0, 0, 160, 200, 155, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182])
             chartData.push([49, 42, 21, 37, 50, 46, 50, 50, 50, 50, 25, 23, 36, 28, 50, 50, 50, 50, 50, 50, 50, 50, 50, 22])
chartData.push([1150, 267, 161, 155, 0, 254, 224, 265, 278, 224])
             chartData.push([1, 10, 41, 49, 50, 21, 27, 22, 22, 23])
chartData.push([86, 0, 0, 54, 56, 0, 0, 0, 48, 0, 0, 75, 85, 62, 0, 0, 73, 0, 0, 0, 0, 83])
             chartData.push([19, 50, 50, 45, 45, 50, 50, 50, 49, 50, 50, 30, 20, 32, 50, 50, 44, 50, 50, 50, 50, 24])
chartData.push([166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110])
             chartData.push([7, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 25])
chartData.push([139, 105, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 107, 0, 136])
             chartData.push([19, 38, 36, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 38, 38, 50, 26])
chartData.push([299])
             chartData.push([27])
chartData.push([148])
             chartData.push([28])
chartData.push([96, 185, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 147])
             chartData.push([48, 18, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 44, 50, 29])
chartData.push([198])
             chartData.push([30])
chartData.push([123, 117, 174, 0, 152, 190, 0, 0, 0, 124, 0, 0, 0, 181, 103, 0, 0, 0, 157, 0, 0, 118, 0, 157])
             chartData.push([30, 36, 17, 50, 16, 15, 50, 50, 50, 31, 50, 50, 50, 18, 48, 50, 50, 50, 40, 50, 50, 47, 50, 31])
chartData.push([142])
             chartData.push([32])
chartData.push([315, 276, 244, 180, 154])
             chartData.push([7, 7, 13, 32, 33])
chartData.push([115])
             chartData.push([34])
chartData.push([159, 0, 0, 0, 153])
             chartData.push([34, 50, 50, 50, 35])
chartData.push([191])
             chartData.push([36])
chartData.push([89])
             chartData.push([37])
chartData.push([157, 213, 280, 146, 120, 181, 214, 192, 132, 209, 0, 0, 130, 139, 106, 85, 0, 107, 129, 0, 0, 157, 151, 126])
             chartData.push([14, 7, 4, 20, 17, 9, 8, 8, 23, 6, 50, 50, 32, 23, 33, 49, 50, 35, 41, 50, 50, 23, 31, 38])
chartData.push([128, 0, 155, 123, 151, 141, 154, 190, 0, 432, 106, 158, 180, 161, 142, 0, 93, 0, 134, 0, 0, 317, 169, 126])
             chartData.push([20, 50, 16, 26, 11, 23, 20, 9, 50, 1, 28, 21, 14, 15, 19, 50, 49, 50, 38, 50, 50, 0, 24, 39])
chartData.push([316, 103, 0, 172, 0, 0, 0, 0, 0, 95, 0, 0, 100, 0, 0, 0, 153, 0, 0, 168, 175, 125])
             chartData.push([2, 31, 50, 10, 50, 50, 50, 50, 50, 48, 50, 50, 39, 50, 50, 50, 26, 50, 50, 21, 21, 40])
chartData.push([2212, 376, 378, 200])
             chartData.push([0, 6, 10, 41])
chartData.push([78, 96, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49])
             chartData.push([10, 7, 3, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 42])
chartData.push([219])
             chartData.push([43])
chartData.push([375, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 0, 0, 116, 0, 0, 0, 109])
             chartData.push([1, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26, 50, 50, 38, 50, 50, 50, 44])
chartData.push([182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 323, 264, 0, 0, 265])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 30, 36, 50, 50, 45])
chartData.push([53, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 56, 73, 92, 60])
             chartData.push([30, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 40, 24, 17, 46])
chartData.push([1402, 248, 241, 222, 153, 176, 0, 0, 0, 143, 0, 161])
             chartData.push([1, 11, 16, 13, 37, 22, 50, 50, 50, 43, 50, 47])
chartData.push([412, 162, 0, 0, 125])
             chartData.push([2, 23, 50, 50, 48])


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