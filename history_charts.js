var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 269])
             chartData.push([41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 0])
chartData.push([341, 491])
             chartData.push([6, 1])
chartData.push([412])
             chartData.push([2])
chartData.push([325])
             chartData.push([3])
chartData.push([38, 0, 35, 0, 0, 73, 103, 173, 0, 0, 55, 0, 128])
             chartData.push([35, 50, 41, 50, 50, 13, 4, 3, 50, 50, 19, 50, 4])
chartData.push([81, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 85, 0, 0, 0, 109, 239])
             chartData.push([43, 50, 50, 50, 50, 50, 50, 50, 39, 50, 50, 50, 32, 50, 50, 50, 36, 5])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6])
chartData.push([315])
             chartData.push([7])
chartData.push([283])
             chartData.push([8])
chartData.push([186, 240])
             chartData.push([20, 9])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10])
chartData.push([90, 172, 0, 0, 0, 0, 0, 127, 0, 324, 227])
             chartData.push([49, 16, 50, 50, 50, 50, 50, 33, 50, 5, 11])
chartData.push([272])
             chartData.push([12])
chartData.push([149, 0, 0, 0, 0, 0, 0, 190, 201, 370, 257, 0, 0, 232, 398, 0, 197, 290])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 22, 21, 6, 16, 50, 50, 14, 4, 50, 39, 13])
chartData.push([101, 0, 162, 162, 89, 194, 135, 113, 75, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 135])
             chartData.push([17, 50, 5, 5, 15, 3, 13, 13, 37, 50, 50, 50, 50, 43, 50, 50, 50, 50, 50, 14])
chartData.push([206])
             chartData.push([15])
chartData.push([319, 193, 124, 188, 138, 115, 129, 96, 0, 247, 0, 0, 0, 169, 112, 140, 167, 265, 171, 187])
             chartData.push([3, 9, 27, 10, 12, 28, 22, 34, 50, 4, 50, 50, 50, 12, 29, 20, 15, 4, 19, 16])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17])
chartData.push([840, 397, 230, 233])
             chartData.push([2, 2, 17, 18])
chartData.push([148, 119])
             chartData.push([12, 19])
chartData.push([32])
             chartData.push([20])
chartData.push([1150, 267, 161, 155, 0, 254])
             chartData.push([1, 10, 41, 49, 50, 21])
chartData.push([136, 0, 190])
             chartData.push([27, 50, 22])
chartData.push([78, 0, 0, 66, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 109])
             chartData.push([26, 50, 50, 40, 50, 50, 41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 49, 23])
chartData.push([113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 280, 0, 0, 184])
             chartData.push([37, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 7, 50, 50, 24])
chartData.push([153])
             chartData.push([25])
chartData.push([168])
             chartData.push([26])
chartData.push([164, 0, 0, 0, 231, 201, 256])
             chartData.push([45, 50, 50, 50, 20, 45, 27])
chartData.push([208, 229, 222, 0, 0, 0, 0, 500, 387, 335, 384, 145, 137, 141, 0, 0, 0, 0, 203, 162])
             chartData.push([11, 8, 10, 50, 50, 50, 50, 2, 4, 3, 4, 29, 34, 27, 50, 50, 50, 50, 16, 28])
chartData.push([265])
             chartData.push([29])
chartData.push([182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 323])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 30])
chartData.push([112, 192, 151, 0, 0, 128])
             chartData.push([26, 6, 14, 50, 50, 31])
chartData.push([106, 0, 0, 0, 245, 288, 201, 167, 129, 140, 0, 0, 0, 192, 162])
             chartData.push([47, 50, 50, 50, 8, 8, 18, 26, 37, 27, 50, 50, 50, 22, 32])
chartData.push([186])
             chartData.push([33])
chartData.push([159])
             chartData.push([34])
chartData.push([196, 955, 556, 170, 143, 168, 0, 144])
             chartData.push([15, 1, 2, 15, 22, 14, 50, 35])
chartData.push([121, 0, 112, 108, 124, 93, 158, 0, 0, 0, 0, 0, 0, 0, 0, 143])
             chartData.push([25, 50, 35, 33, 33, 46, 19, 50, 50, 50, 50, 50, 50, 50, 50, 36])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37])
chartData.push([375, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 0, 0, 116])
             chartData.push([1, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26, 50, 50, 38])
chartData.push([344, 194, 165, 159, 206, 205, 138, 152, 0, 0, 0, 171, 114, 192, 186, 0, 151, 136, 246, 148])
             chartData.push([4, 14, 20, 22, 7, 11, 27, 22, 50, 50, 50, 25, 49, 16, 17, 50, 25, 28, 14, 39])
chartData.push([115, 0, 182, 206, 167, 271, 330, 353, 134])
             chartData.push([40, 50, 13, 13, 17, 6, 3, 4, 40])
chartData.push([395, 201])
             chartData.push([9, 41])
chartData.push([67, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 70])
             chartData.push([30, 50, 50, 50, 32, 50, 50, 50, 50, 50, 50, 50, 34, 50, 50, 50, 50, 50, 42])
chartData.push([103, 0, 0, 0, 0, 0, 0, 125, 245, 107, 0, 0, 0, 0, 0, 91])
             chartData.push([14, 50, 50, 50, 50, 50, 50, 22, 4, 25, 50, 50, 50, 50, 50, 43])
chartData.push([79])
             chartData.push([44])
chartData.push([324, 0, 0, 167, 273, 137, 0, 0, 0, 0, 0, 0, 0, 0, 140, 140, 0, 0, 158])
             chartData.push([6, 50, 50, 18, 8, 40, 50, 50, 50, 50, 50, 50, 50, 50, 36, 42, 50, 50, 45])
chartData.push([135])
             chartData.push([46])
chartData.push([66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56])
             chartData.push([24, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 47])
chartData.push([59, 0, 0, 70, 0, 0, 70, 0, 74, 105, 128, 89, 71, 0, 74, 101, 77])
             chartData.push([48, 50, 50, 38, 50, 50, 32, 50, 39, 23, 8, 25, 33, 50, 37, 29, 48])


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